//! Zom is a AOT compiled programming language.
//!
//! The front-end (lexer, parser, token, ast) is in the crate `zom_fe`.
//! Custom errors are in crate `zom_common`.
//!
//! Zom repository, <https://github.com/zom-lang/zom>

mod ops;

use std::{error::Error, ffi::OsString};

use clap::{Parser, Subcommand};
use ops::{bobj, gettarget::gettarget, version};

#[derive(Debug)]
struct SError {
    msg: String,
}

impl SError {
    fn new<S: Into<String>>(msg: S) -> Self {
        SError { msg: msg.into() }
    }
}

impl Error for SError {}

impl std::fmt::Display for SError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[macro_export]
macro_rules! err {
    (raw $msg:expr) => ({
        use $crate::SError;
        Box::new(SError::new($msg))
    });

    ($msg:expr) => (
        Err(err!(raw $msg))
    );

    (fmt $msg:tt $(, $arg:expr)*) => ({
        use $crate::SError;
        Err(Box::new(SError::new(format!($msg, $( $arg ),* ))))
    });
}

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

    /// A subcommand used when devlopment, to quickly access a function or thing like that.
    #[cfg(debug_assertions)]
    Dev,
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

pub fn run_with_args<T, I>(args: I) -> Result<ExitStatus, Box<dyn Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = Args::parse_from(args);
    match args.command {
        Command::Bobj(args) => bobj::build(args),
        Command::Version => version::version(),
        Command::GetTarget => gettarget(),
        #[cfg(debug_assertions)]
        Command::Dev => ops::dev::dev(),
    }
}
