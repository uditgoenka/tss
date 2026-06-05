use super::command::CommandSpec;
use super::shell::ShellCommandExt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

static RAW_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub combined: Vec<u8>,
    pub exit_code: i32,
}

impl RawOutput {
    pub fn new(stdout: Vec<u8>, stderr: Vec<u8>, exit_code: i32) -> Self {
        let mut combined = Vec::with_capacity(stdout.len() + stderr.len());
        combined.extend_from_slice(&stdout);
        combined.extend_from_slice(&stderr);
        Self {
            stdout,
            stderr,
            combined,
            exit_code,
        }
    }

    pub fn from_parts(stdout: Vec<u8>, stderr: Vec<u8>, combined: Vec<u8>, exit_code: i32) -> Self {
        Self {
            stdout,
            stderr,
            combined,
            exit_code,
        }
    }

    pub fn byte_len(&self) -> usize {
        self.stdout.len() + self.stderr.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawRecord {
    pub id: String,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub combined: Vec<u8>,
    pub exit_code: i32,
    pub cwd: PathBuf,
    pub command_hash: String,
    pub timestamp_unix_millis: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawRenderMode {
    Full,
    Stdout,
    Stderr,
    Combined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawStore {
    root: PathBuf,
}

impl RawStore {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub fn store(&self, command: &CommandSpec, output: &RawOutput) -> io::Result<RawRecord> {
        if raw_store_disabled() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "raw output storage disabled by TSS_NO_STORE",
            ));
        }

        ensure_private_dir(&self.root)?;

        let timestamp_unix_millis = now_unix_millis();
        let id = generate_raw_id(output, timestamp_unix_millis);
        let record = RawRecord {
            id,
            stdout: output.stdout.clone(),
            stderr: output.stderr.clone(),
            combined: output.combined.clone(),
            exit_code: output.exit_code,
            cwd: std::env::current_dir()?,
            command_hash: command.command_hash(),
            timestamp_unix_millis,
        };

        let path = self.record_path(&record.id);
        let mut file = secure_create(&path)?;
        file.write_all(serialize_record(&record).as_bytes())?;
        file.flush()?;

        Ok(record)
    }

    pub fn get(&self, id: &str) -> io::Result<RawRecord> {
        if !is_valid_raw_id(id) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid raw output id",
            ));
        }

        let encoded = fs::read_to_string(self.record_path(id))?;
        deserialize_record(&encoded)
    }

    pub fn render(&self, id: &str, mode: RawRenderMode) -> io::Result<Vec<u8>> {
        let record = self.get(id)?;
        let rendered = match mode {
            RawRenderMode::Stdout => record.stdout,
            RawRenderMode::Stderr => record.stderr,
            RawRenderMode::Combined => record.combined,
            RawRenderMode::Full => render_full_record(&record),
        };
        Ok(rendered)
    }

    fn record_path(&self, id: &str) -> PathBuf {
        self.root.join(format!("{id}.raw"))
    }
}

fn raw_store_disabled() -> bool {
    std::env::var("TSS_NO_STORE")
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

fn secure_create(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        options.mode(0o600);
    }
    options.open(path)
}

fn ensure_private_dir(path: &Path) -> io::Result<()> {
    let missing = missing_ancestors(path);
    fs::create_dir_all(path)?;
    for directory in missing {
        set_private_dir_mode(&directory)?;
    }
    Ok(())
}

fn missing_ancestors(path: &Path) -> Vec<PathBuf> {
    let mut missing = Vec::new();
    let mut current = Some(path);
    while let Some(candidate) = current {
        if candidate.as_os_str().is_empty() {
            break;
        }
        if candidate.exists() {
            break;
        }
        missing.push(candidate.to_path_buf());
        current = candidate.parent();
    }
    missing.reverse();
    missing
}

#[cfg(unix)]
fn set_private_dir_mode(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(path, permissions)
}

#[cfg(not(unix))]
fn set_private_dir_mode(_path: &Path) -> io::Result<()> {
    Ok(())
}

fn now_unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn generate_raw_id(output: &RawOutput, timestamp_unix_millis: u128) -> String {
    let counter = RAW_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut hash = 0xcbf29ce484222325u64;
    hash = fnv1a(hash, &timestamp_unix_millis.to_le_bytes());
    hash = fnv1a(hash, &counter.to_le_bytes());
    hash = fnv1a(hash, &output.exit_code.to_le_bytes());
    hash = fnv1a(hash, &output.stdout);
    hash = fnv1a(hash, &output.stderr);
    hash = fnv1a(hash, &output.combined);
    format!("tssr_{timestamp_unix_millis:x}_{counter:x}_{hash:016x}")
}

fn fnv1a(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn is_valid_raw_id(id: &str) -> bool {
    id.starts_with("tssr_")
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn serialize_record(record: &RawRecord) -> String {
    let mut encoded = String::new();
    encoded.push_str("TSSRAW1\n");
    push_field(&mut encoded, "id", &record.id);
    push_field(&mut encoded, "exit_code", &record.exit_code.to_string());
    push_field(
        &mut encoded,
        "timestamp_unix_millis",
        &record.timestamp_unix_millis.to_string(),
    );
    push_field(&mut encoded, "command_hash", &record.command_hash);
    push_field(
        &mut encoded,
        "cwd",
        &hex_encode(record.cwd.to_string_lossy().as_bytes()),
    );
    push_field(&mut encoded, "stdout", &hex_encode(&record.stdout));
    push_field(&mut encoded, "stderr", &hex_encode(&record.stderr));
    push_field(&mut encoded, "combined", &hex_encode(&record.combined));
    encoded
}

fn push_field(encoded: &mut String, key: &str, value: &str) {
    encoded.push_str(key);
    encoded.push('=');
    encoded.push_str(value);
    encoded.push('\n');
}

fn deserialize_record(encoded: &str) -> io::Result<RawRecord> {
    let mut lines = encoded.lines();
    if lines.next() != Some("TSSRAW1") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid raw output record",
        ));
    }

    let mut id = None;
    let mut exit_code = None;
    let mut timestamp_unix_millis = None;
    let mut command_hash = None;
    let mut cwd = None;
    let mut stdout = None;
    let mut stderr = None;
    let mut combined = None;

    for line in lines {
        let Some((key, value)) = line.split_once('=') else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid raw output field",
            ));
        };

        match key {
            "id" => id = Some(value.to_string()),
            "exit_code" => {
                exit_code = Some(value.parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "invalid raw exit code")
                })?)
            }
            "timestamp_unix_millis" => {
                timestamp_unix_millis = Some(value.parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "invalid raw timestamp")
                })?)
            }
            "command_hash" => command_hash = Some(value.to_string()),
            "cwd" => {
                cwd = Some(PathBuf::from(
                    String::from_utf8(hex_decode(value)?).map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid raw cwd")
                    })?,
                ))
            }
            "stdout" => stdout = Some(hex_decode(value)?),
            "stderr" => stderr = Some(hex_decode(value)?),
            "combined" => combined = Some(hex_decode(value)?),
            _ => {}
        }
    }

    Ok(RawRecord {
        id: required(id, "id")?,
        stdout: required(stdout, "stdout")?,
        stderr: required(stderr, "stderr")?,
        combined: required(combined, "combined")?,
        exit_code: required(exit_code, "exit_code")?,
        cwd: required(cwd, "cwd")?,
        command_hash: required(command_hash, "command_hash")?,
        timestamp_unix_millis: required(timestamp_unix_millis, "timestamp_unix_millis")?,
    })
}

fn required<T>(value: Option<T>, field: &'static str) -> io::Result<T> {
    value.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("missing raw output field: {field}"),
        )
    })
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[(byte >> 4) as usize]));
        encoded.push(char::from(HEX[(byte & 0x0f) as usize]));
    }
    encoded
}

fn hex_decode(encoded: &str) -> io::Result<Vec<u8>> {
    if !encoded.len().is_multiple_of(2) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid hex field length",
        ));
    }

    let mut bytes = Vec::with_capacity(encoded.len() / 2);
    for pair in encoded.as_bytes().chunks_exact(2) {
        let high = hex_value(pair[0])?;
        let low = hex_value(pair[1])?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn hex_value(byte: u8) -> io::Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid hex character",
        )),
    }
}

fn render_full_record(record: &RawRecord) -> Vec<u8> {
    let mut rendered = format!(
        "id: {}\nexit_code: {}\ncwd: {}\ncommand_hash: {}\ntimestamp_unix_millis: {}\n\n--- stdout ---\n",
        record.id,
        record.exit_code,
        record.cwd.display(),
        record.command_hash,
        record.timestamp_unix_millis
    )
    .into_bytes();
    rendered.extend_from_slice(&record.stdout);
    rendered.extend_from_slice(b"\n--- stderr ---\n");
    rendered.extend_from_slice(&record.stderr);
    rendered
}
