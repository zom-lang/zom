//! Mona is a JIT compiled programming language. The binary implements a REPL and soon; a file reader.
//!
//! The front-end (lexer, parser, token, ast) is in `src/fe/`.
//!
//! Custom errors are in module `error` in `src/error/`.
//! A REPL for Mona is in the `driver` module in `src/driver.rs`.
//!
//! Mona repository link -> <https://github.com/Larsouille25/mona>

pub mod driver;
pub mod error;

pub mod fe;

pub mod me;

pub mod be;
