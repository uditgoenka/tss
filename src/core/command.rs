#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
}

impl CommandSpec {
    pub fn new(program: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            program: program.into(),
            args,
        }
    }

    pub fn from_run_args(args: Vec<String>) -> Result<Self, &'static str> {
        let command_args = match args.first().map(String::as_str) {
            Some("--") => &args[1..],
            _ => &args[..],
        };

        let Some((program, program_args)) = command_args.split_first() else {
            return Err("usage: tss run -- <cmd> [args...]");
        };

        Ok(Self::new(program.clone(), program_args.to_vec()))
    }
}
