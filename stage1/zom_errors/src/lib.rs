use std::fmt;
use std::{ops::Range, path::PathBuf};

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use lazy_static::lazy_static;

pub mod err;
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
    pub fn build_log<L: Log>(&self, log: L) -> BuiltLog {
        log.build(self)
    }

    pub fn add<L: Log>(&mut self, log: L) {
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

    pub fn stream(&self) -> Vec<BuiltLog> {
        self.logs.clone()
    }
}

pub fn format_tokens(tokens: &Vec<FmtToken>) -> String {
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

pub trait Log {
    /// location of the found token on the line
    fn location(&self) -> CodeSpan;

    /// level of the log
    fn level(&self) -> LogLevel;

    /// message of the log
    fn msg(&self) -> String;

    /// note message
    fn note(&self) -> Option<String> {
        None
    }

    /// build the log using a LogContext.
    /// It's prefered to call the `build_log` method on LogContext instead of calling this method.
    fn build(&self, ctx: &LogContext) -> BuiltLog {
        let loc = ctx.location(self.location().clone());
        let loc_end = ctx.location(self.location().end..self.location().end + 1);
        BuiltLog {
            file_path: ctx.file_path.clone(),
            loc: loc.clone(),
            lvl: self.level(),
            msg: self.msg(),
            note: self.note(),
            code: ctx.get_line(loc.clone()),
            span: loc.col..loc_end.col,
        }
    }
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

#[derive(Debug, Clone)]
pub struct BuiltLog {
    file_path: PathBuf,
    /// location of the log inside the file
    loc: CodeLocation,
    lvl: LogLevel,
    /// log message
    msg: String,
    /// note message to specify the log message
    note: Option<String>,
    /// code at the line of the log
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
        write!(
            s,
            "{}{}",
            spaces(self.loc.col - 1),
            repeat('^', self.span.end - self.span.start),
        )?;
        if let Some(note) = &self.note {
            write!(s, " {note}")?;
        }
        writeln!(s, "")?;
        s.reset()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
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

pub enum FinalRes<T> {
    Ok(T, Vec<BuiltLog>),
    Err(Vec<BuiltLog>),
}
