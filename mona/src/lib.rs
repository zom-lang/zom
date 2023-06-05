//! Mona is a AOT compiled programming language. The binary implements a REPL and soon; a file reader.
//!
//! The front-end (lexer, parser, token, ast) is in the crate `mona_fe`.
//!
//! Custom errors are in module `error` in `src/error/`.
//! A REPL for Mona is in the `driver` module in `src/driver.rs`.
//!
//! Mona repository link -> <https://github.com/Larsouille25/mona>

mod ops;

use std::ffi::OsString;

use clap::{Parser, Subcommand};
use ops::bobj;

#[derive(Parser)]
#[clap()]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Builds a given file into an object file
    Bobj(bobj::Args),
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum ExitStatus {
    Success,
    Error,
}

impl From<bool> for ExitStatus {
    fn from(value: bool) -> Self {
        if value {
            ExitStatus::Success
        } else {
            ExitStatus::Error
        }
    }
}

pub fn run_with_args<T, I>(args: I) -> Result<ExitStatus, anyhow::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = Args::parse_from(args);
    match args.command {
        Command::Bobj(args) => bobj::build(args),
    }
}
