//! Zom is a AOT compiled programming language.
//!
//! The front-end (lexer, parser, token, ast) is in the crate `zom_fe`.
//! Custom errors are in crate `zom_common`.
//!
//! Zom repository, <https://github.com/zom-lang/zom>

mod ops;

use std::ffi::OsString;

use clap::{Parser, Subcommand};
use ops::{bobj, gettarget::gettarget, version};

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

    /// Get the current version of Zom
    Version,

    /// Get the current target detected by LLVM.
    GetTarget,
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
        Command::Version => version::version(),
        Command::GetTarget => gettarget(),
    }
}
