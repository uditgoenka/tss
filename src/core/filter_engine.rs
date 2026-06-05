use super::command::CommandSpec;
use super::policy::{SafetyDecision, SafetyGate, Support};
use super::raw_store::{RawOutput, RawStore};
use std::io;

pub trait OutputFilter {
    fn name(&self) -> &'static str;
    fn supports(&self, command: &CommandSpec) -> Support;
    fn filter(&self, raw: RawOutput, context: FilterContext) -> FilterOutcome;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterContext {
    pub command: CommandSpec,
    pub raw_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterOutcome {
    pub output: String,
    pub exit_code: i32,
    pub omitted_bytes: usize,
    pub omissions: usize,
    pub lossy: bool,
    pub structured_format: Option<StructuredFormat>,
}

impl FilterOutcome {
    pub fn lossless(output: impl Into<String>, exit_code: i32) -> Self {
        Self {
            output: output.into(),
            exit_code,
            omitted_bytes: 0,
            omissions: 0,
            lossy: false,
            structured_format: None,
        }
    }

    pub fn lossy(
        output: impl Into<String>,
        exit_code: i32,
        omitted_bytes: usize,
        omissions: usize,
    ) -> Self {
        Self {
            output: output.into(),
            exit_code,
            omitted_bytes,
            omissions,
            lossy: true,
            structured_format: None,
        }
    }

    pub fn requiring_structured_format(mut self, format: StructuredFormat) -> Self {
        self.structured_format = Some(format);
        self
    }

    fn rendered_bytes(&self) -> usize {
        self.output.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuredFormat {
    Json,
    Diff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    ExitCodeChanged {
        raw: i32,
        filtered: i32,
    },
    NonZeroExitLooksSuccessful,
    BytesRemovedWithoutLossyMarker {
        raw_bytes: usize,
        filtered_bytes: usize,
    },
    LossyOutputMissingOmissionAccounting,
    InvalidStructuredOutput(StructuredFormat),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Validator;

impl Validator {
    pub fn validate(
        &self,
        raw: &RawOutput,
        filtered: &FilterOutcome,
    ) -> Result<(), ValidationError> {
        if raw.exit_code != filtered.exit_code {
            return Err(ValidationError::ExitCodeChanged {
                raw: raw.exit_code,
                filtered: filtered.exit_code,
            });
        }

        if raw.exit_code != 0 && looks_like_success_only(&filtered.output) {
            return Err(ValidationError::NonZeroExitLooksSuccessful);
        }

        if let Some(format) = filtered.structured_format {
            match format {
                StructuredFormat::Json if !is_valid_json(&filtered.output) => {
                    return Err(ValidationError::InvalidStructuredOutput(format));
                }
                StructuredFormat::Json => {}
                StructuredFormat::Diff if !is_valid_unified_diff(&filtered.output) => {
                    return Err(ValidationError::InvalidStructuredOutput(format));
                }
                StructuredFormat::Diff => {}
            }
        }

        let raw_bytes = raw.byte_len();
        let filtered_bytes = filtered.rendered_bytes();
        if filtered_bytes < raw_bytes && !filtered.lossy {
            return Err(ValidationError::BytesRemovedWithoutLossyMarker {
                raw_bytes,
                filtered_bytes,
            });
        }

        if filtered.lossy && (filtered.omitted_bytes == 0 || filtered.omissions == 0) {
            return Err(ValidationError::LossyOutputMissingOmissionAccounting);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineEmission {
    pub output: Vec<u8>,
    pub exit_code: i32,
    pub raw_id: Option<String>,
    pub decision: SafetyDecision,
    pub lossy: bool,
    pub omissions: usize,
}

pub struct FilterEngine<'a> {
    gate: SafetyGate,
    raw_store: &'a RawStore,
    validator: Validator,
}

impl<'a> FilterEngine<'a> {
    pub fn new(raw_store: &'a RawStore) -> Self {
        Self {
            gate: SafetyGate::default(),
            raw_store,
            validator: Validator,
        }
    }

    pub fn with_gate(raw_store: &'a RawStore, gate: SafetyGate) -> Self {
        Self {
            gate,
            raw_store,
            validator: Validator,
        }
    }

    pub fn process(
        &self,
        command: &CommandSpec,
        raw: RawOutput,
        filter: &dyn OutputFilter,
    ) -> io::Result<EngineEmission> {
        let stored = self.raw_store.store(command, &raw)?;
        let support = filter.supports(command);
        let decision = self.gate.decide(command, support);

        if decision != SafetyDecision::FilterAllowed {
            return Ok(EngineEmission {
                output: raw.combined,
                exit_code: raw.exit_code,
                raw_id: Some(stored.id),
                decision,
                lossy: false,
                omissions: 0,
            });
        }

        let context = FilterContext {
            command: command.clone(),
            raw_id: stored.id.clone(),
        };
        let filtered = filter.filter(raw.clone(), context);

        if self.validator.validate(&raw, &filtered).is_err() {
            return Ok(EngineEmission {
                output: raw.combined,
                exit_code: raw.exit_code,
                raw_id: Some(stored.id),
                decision: SafetyDecision::PassthroughUnsafe("filter output failed validation"),
                lossy: false,
                omissions: 0,
            });
        }

        let output = render_filtered_output(&filtered, &stored.id);
        Ok(EngineEmission {
            output,
            exit_code: filtered.exit_code,
            raw_id: Some(stored.id),
            decision,
            lossy: filtered.lossy,
            omissions: filtered.omissions,
        })
    }
}

fn render_filtered_output(filtered: &FilterOutcome, raw_id: &str) -> Vec<u8> {
    if !filtered.lossy {
        return filtered.output.as_bytes().to_vec();
    }

    let mut output = filtered.output.clone();
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output.push_str(&format!(
        "[tss: omitted {} bytes across {} omission(s); recover with tss raw {}]\n",
        filtered.omitted_bytes, filtered.omissions, raw_id
    ));
    output.into_bytes()
}

fn looks_like_success_only(output: &str) -> bool {
    let lower = output.to_ascii_lowercase();
    let success_words = ["pass", "passed", "success", "succeeded", "ok"];
    let failure_words = ["fail", "failed", "error", "panic", "non-zero", "nonzero"];

    success_words.iter().any(|word| lower.contains(word))
        && !failure_words.iter().any(|word| lower.contains(word))
}

fn is_valid_json(input: &str) -> bool {
    let mut parser = JsonParser::new(input);
    parser.parse_value() && parser.remaining_is_whitespace()
}

fn is_valid_unified_diff(input: &str) -> bool {
    let mut saw_file_header = false;
    let mut saw_hunk = false;

    for line in input.lines() {
        if line.starts_with("diff --git ")
            || line.starts_with("index ")
            || line.starts_with("new file mode ")
            || line.starts_with("deleted file mode ")
            || line.starts_with("similarity index ")
            || line.starts_with("rename from ")
            || line.starts_with("rename to ")
        {
            continue;
        }

        if line.starts_with("--- ") || line.starts_with("+++ ") {
            saw_file_header = true;
            continue;
        }

        if line.starts_with("@@ ") {
            saw_hunk = true;
            continue;
        }

        if saw_hunk
            && (line.starts_with('+')
                || line.starts_with('-')
                || line.starts_with(' ')
                || line == "\\ No newline at end of file")
        {
            continue;
        }

        return false;
    }

    saw_file_header && saw_hunk
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            index: 0,
        }
    }

    fn parse_value(&mut self) -> bool {
        self.skip_whitespace();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => self.parse_string(),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            Some(b't') => self.consume_literal(b"true"),
            Some(b'f') => self.consume_literal(b"false"),
            Some(b'n') => self.consume_literal(b"null"),
            _ => false,
        }
    }

    fn parse_object(&mut self) -> bool {
        if !self.consume_byte(b'{') {
            return false;
        }
        self.skip_whitespace();
        if self.consume_byte(b'}') {
            return true;
        }

        loop {
            self.skip_whitespace();
            if !self.parse_string() {
                return false;
            }
            self.skip_whitespace();
            if !self.consume_byte(b':') {
                return false;
            }
            if !self.parse_value() {
                return false;
            }
            self.skip_whitespace();
            if self.consume_byte(b'}') {
                return true;
            }
            if !self.consume_byte(b',') {
                return false;
            }
        }
    }

    fn parse_array(&mut self) -> bool {
        if !self.consume_byte(b'[') {
            return false;
        }
        self.skip_whitespace();
        if self.consume_byte(b']') {
            return true;
        }

        loop {
            if !self.parse_value() {
                return false;
            }
            self.skip_whitespace();
            if self.consume_byte(b']') {
                return true;
            }
            if !self.consume_byte(b',') {
                return false;
            }
        }
    }

    fn parse_string(&mut self) -> bool {
        if !self.consume_byte(b'"') {
            return false;
        }

        while let Some(byte) = self.next() {
            match byte {
                b'"' => return true,
                b'\\' => {
                    let Some(escaped) = self.next() else {
                        return false;
                    };
                    if matches!(
                        escaped,
                        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't'
                    ) {
                        continue;
                    }
                    if escaped == b'u' && self.consume_hex_quad() {
                        continue;
                    }
                    return false;
                }
                0x00..=0x1f => return false,
                _ => {}
            }
        }

        false
    }

    fn parse_number(&mut self) -> bool {
        if self.peek() == Some(b'-') {
            self.index += 1;
        }

        match self.peek() {
            Some(b'0') => self.index += 1,
            Some(b'1'..=b'9') => {
                self.index += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.index += 1;
                }
            }
            _ => return false,
        }

        if self.peek() == Some(b'.') {
            self.index += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return false;
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.index += 1;
            }
        }

        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.index += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.index += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return false;
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.index += 1;
            }
        }

        true
    }

    fn consume_literal(&mut self, literal: &[u8]) -> bool {
        if self.bytes.get(self.index..self.index + literal.len()) == Some(literal) {
            self.index += literal.len();
            true
        } else {
            false
        }
    }

    fn consume_hex_quad(&mut self) -> bool {
        for _ in 0..4 {
            match self.next() {
                Some(byte) if byte.is_ascii_hexdigit() => {}
                _ => return false,
            }
        }
        true
    }

    fn remaining_is_whitespace(&mut self) -> bool {
        self.skip_whitespace();
        self.index == self.bytes.len()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.index += 1;
        }
    }

    fn consume_byte(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.index += 1;
        Some(byte)
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }
}
