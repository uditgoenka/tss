use std::io;
use std::process::Command;
use std::time::{Duration, Instant};

use super::CommandSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub combined: Vec<u8>,
    pub exit_code: i32,
    pub duration: Duration,
}

#[derive(Debug)]
pub enum RunnerError {
    Spawn { program: String, source: io::Error },
}

pub struct PassthroughRunner;

impl PassthroughRunner {
    pub fn run(spec: &CommandSpec) -> Result<RawOutput, RunnerError> {
        let started_at = Instant::now();
        let output = Command::new(&spec.program)
            .args(&spec.args)
            .output()
            .map_err(|source| RunnerError::Spawn {
                program: spec.program.clone(),
                source,
            })?;

        let mut combined = Vec::with_capacity(output.stdout.len() + output.stderr.len());
        combined.extend_from_slice(&output.stdout);
        combined.extend_from_slice(&output.stderr);

        Ok(RawOutput {
            stdout: output.stdout,
            stderr: output.stderr,
            combined,
            exit_code: output.status.code().unwrap_or(1),
            duration: started_at.elapsed(),
        })
    }
}
