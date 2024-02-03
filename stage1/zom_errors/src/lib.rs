use std::fmt;
use std::{ops::Range, path::PathBuf};

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use lazy_static::lazy_static;

pub mod prelude;

lazy_static! {
    static ref BOLD_STYLE: ColorSpec = ColorSpec::new().set_bold(true).clone();
    static ref RED_STYLE: ColorSpec = ColorSpec::new()
        .set_bold(true)
        .set_fg(Some(Color::Red))
        .clone();
    static ref MAGENTA_STYLE: ColorSpec = ColorSpec::new()
        .set_bold(true)
        .set_fg(Some(Color::Magenta))
        .clone();
    static ref BLUE_STYLE: ColorSpec = ColorSpec::new()
        .set_bold(true)
        .set_fg(Some(Color::Blue))
        .clone();
}

pub enum FmtToken {
    Operator,

    // Delimiter
    OpenParen,
    CloseParen,

    OpenBracket,
    CloseBracket,

    OpenBrace,
    CloseBrace,

    // Punctuation
    Colon,
    SemiColon,
    Comma,
    At,

    // Literals
    IntLit,
    FloatLit,
    StrLit,
    CharLit,

    // Keywords
    Fn,
    Extern,
    Var,
    Const,
    Struct,
    Enum,
    Return,
    If,
    Else,
    While,
    For,
    Pub,
    Async,
    Await,
    Match,
    Impl,
    True,
    False,
    Undefined,
    Break,
    Continue,

    Ident,

    EOF,
}

impl fmt::Display for FmtToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FmtToken::*;
        write!(
            f,
            "{}",
            match self {
                Operator => "operator",
                OpenParen => "(",
                CloseParen => ")",
                OpenBracket => "[",
                CloseBracket => "]",
                OpenBrace => "{",
                CloseBrace => "}",
                Colon => ",",
                SemiColon => ";",
                Comma => ",",
                At => "@",
                IntLit => "integer literal",
                FloatLit => "float literal",
                StrLit => "string literal",
                CharLit => "char literal",
                Fn => "'fn'",
                Extern => "'extern'",
                Var => "'var'",
                Const => "'const'",
                Struct => "'struct'",
                Enum => "'enum'",
                Return => "'return'",
                If => "'if'",
                Else => "'else'",
                While => "'while'",
                For => "'for'",
                Pub => "'pub'",
                Async => "'async'",
                Await => "'await'",
                Match => "'match'",
                Impl => "'impl'",
                True => "'true'",
                False => "'false'",
                Undefined => "'undefined'",
                Break => "'break'",
                Continue => "'continue'",
                Ident => "identifier",
                EOF => "end of file",
            }
        )
    }
}

pub type CodeSpan = Range<usize>;

#[derive(Debug)]
pub struct LogContext {
    file: String,
    file_path: PathBuf,
    logs: Vec<BuiltLog>,
    color: ColorChoice,
}

impl LogContext {
    pub fn new(file: String, file_path: PathBuf, color: ColorChoice) -> LogContext {
        Self::with_logs(file, file_path, color, Vec::new())
    }

    pub fn with_logs(
        file: String,
        file_path: PathBuf,
        color: ColorChoice,
        logs: Vec<BuiltLog>,
    ) -> LogContext {
        LogContext {
            file,
            file_path,
            logs,
            color,
        }
    }

    /// Add a BuildLog to the error stream
    pub fn add_raw(&mut self, blog: BuiltLog) {
        self.logs.push(blog);
    }

    /// Returns true if their is at least one `Log` with `LogLevel` of `Error`, instead false.
    pub fn failed(&self) -> bool {
        for log in &self.logs {
            if let LogLevel::Error = log.lvl {
                return true;
            }
        }
        return false;
    }

    pub fn location(&self, span: CodeSpan) -> CodeLocation {
        assert!(
            span.start < span.end,
            "The start of the range is greater than its end."
        );

        let mut line = 1;
        let mut col = 1;

        for (idx, ch) in self.file.char_indices() {
            if span.start == idx {
                break;
            }
            match ch {
                '\n' => {
                    col = 1;
                    line += 1;
                }
                _ => col += 1,
            }
        }

        return CodeLocation { col, line };
    }

    /// Get the line content with a given `CodeLocation`
    pub fn get_line(&self, loc: CodeLocation) -> String {
        self.file.lines().nth(loc.line - 1).unwrap().to_string()
    }

    /// Build a `Log` into a `BuiltLog`
    pub fn build_log(&self, log: Log) -> BuiltLog {
        match log {
            Log::ExpectedToken {
                expected,
                found,
                location,
            } => {
                let loc = self.location(location.clone());
                let loc_end = self.location(location.end..location.end + 1);
                BuiltLog {
                    file_path: self.file_path.clone(),
                    loc: loc.clone(),
                    lvl: LogLevel::Error,
                    msg: format!("expected {}, found {}", format_tokens(&expected), found),
                    code: self.get_line(loc.clone()),
                    span: loc.col..loc_end.col,
                }
            }
            Log::UnclosedDelimiter {
                delimiter,
                location,
            } => {
                let loc = self.location(location.clone());
                let loc_end = self.location(location.end..location.end + 1);
                BuiltLog {
                    file_path: self.file_path.clone(),
                    loc: loc.clone(),
                    lvl: LogLevel::Error,
                    msg: format!("unclosed delimiter `{}`", delimiter),
                    code: self.get_line(loc.clone()),
                    span: loc.col..loc_end.col,
                }
            }
        }
    }

    pub fn add(&mut self, log: Log) {
        self.logs.push(self.build_log(log))
    }

    pub fn print(&self) {
        let mut stdout = StandardStream::stdout(self.color);
        self.format(&mut stdout).expect("error formating failed.");
    }

    pub fn format(&self, s: &mut StandardStream) -> Result<(), io::Error> {
        let len = self.logs.len();
        for (i, log) in self.logs.iter().enumerate() {
            log.format(s)?;

            if i != len - 1 {
                writeln!(s)?;
            }
        }
        Ok(())
    }

    pub fn log_stream(&self) -> &Vec<BuiltLog> {
        &self.logs
    }
}

fn format_tokens(tokens: &Vec<FmtToken>) -> String {
    if tokens.len() == 1 {
        format!("{}", tokens[0])
    } else {
        let mut s = String::new();
        for (len, tt) in tokens.iter().enumerate() {
            if len == tokens.len() - 2 {
                s.push_str(&format!("{} ", tt));
                continue;
            } else if len == tokens.len() - 1 {
                s.push_str(&format!("or {}", tt));
                continue;
            }
            s.push_str(&format!("{}, ", tt))
        }
        s
    }
}

#[derive(Clone, Debug)]
pub struct CodeLocation {
    pub col: usize,
    pub line: usize,
}

pub enum Log {
    ExpectedToken {
        /// list of expected tokens
        expected: Vec<FmtToken>,
        /// token found
        found: FmtToken,
        /// location of the found token
        location: CodeSpan,
    },
    UnclosedDelimiter {
        /// expected closing delimiter
        delimiter: FmtToken,
        /// location of the opening delimiter
        location: CodeSpan,
    },
}

/// Repeats a char (c), n times and returns it.
fn repeat(c: char, n: usize) -> String {
    let mut s = String::with_capacity(n);
    for _ in 0..n {
        s.push(c);
    }
    s
}

/// Repeats spaces n times and returns a string containing it.
fn spaces(n: usize) -> String {
    repeat(' ', n)
}

#[derive(Debug)]
pub struct BuiltLog {
    file_path: PathBuf,
    loc: CodeLocation,
    lvl: LogLevel,
    msg: String,
    code: String,
    span: CodeSpan,
}

impl BuiltLog {
    pub fn format(&self, s: &mut StandardStream) -> Result<(), io::Error> {
        s.set_color(&BOLD_STYLE)?;
        write!(
            s,
            "{}:{}:{}: ",
            self.file_path.clone().into_os_string().to_str().unwrap(),
            self.loc.line,
            self.loc.col,
        )?;
        self.lvl.format(s)?;
        s.set_color(&BOLD_STYLE)?;
        writeln!(s, ": {}", self.msg)?;
        s.reset()?;
        writeln!(s, "{}", self.code)?;
        s.set_color(&BLUE_STYLE)?;
        writeln!(
            s,
            "{}{}",
            spaces(self.loc.col - 1),
            repeat('^', self.span.end - self.span.start)
        )?;
        s.reset()?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum LogLevel {
    Warning,
    Error,
}

impl LogLevel {
    pub fn format(&self, s: &mut StandardStream) -> Result<(), io::Error> {
        match self {
            Self::Warning => {
                s.set_color(&MAGENTA_STYLE)?;
                write!(s, "warning")
            }
            Self::Error => {
                s.set_color(&RED_STYLE)?;
                write!(s, "error")
            }
        }
    }
}
