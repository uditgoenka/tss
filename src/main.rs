pub mod analytics;
mod cli;
pub mod core;
pub mod filters;
pub mod integrations;
pub mod privacy;

use std::{env, process};

fn main() {
    process::exit(cli::run(env::args().skip(1)));
}
