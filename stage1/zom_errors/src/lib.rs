use std::fmt;
use std::{
    ops::Range,
    path::{Path, PathBuf},
};

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use lazy_static::lazy_static;

use zom_common::token::Operator;
use zom_common::token::Token;

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

#[derive(Debug)]
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
    Dot,

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
    Package,
    Import,
    As,

    Ident,

    EOF,
}

impl FmtToken {
    pub fn from_token(token: &Token) -> Self {
        use crate::FmtToken::*;
        use zom_common::token::Operator;
        use zom_common::token::TokenType as TT;
        let tt = &token.tt;
        match tt {
            TT::Oper(Operator::Dot) => Dot,
            TT::Oper(_) => Operator,

            TT::OpenParen => OpenParen,
            TT::CloseParen => CloseParen,

            TT::OpenBracket => OpenBracket,
            TT::CloseBracket => CloseBracket,

            TT::OpenBrace => OpenBrace,
            TT::CloseBrace => CloseBrace,

            TT::Colon => Colon,
            TT::SemiColon => SemiColon,
            TT::Comma => Comma,
            TT::At => At,

            TT::Int(_) => IntLit,
            TT::Float(_) => FloatLit,
            TT::Str(_) => StrLit,
            TT::Char(_) => CharLit,

            TT::Fn => Fn,
            TT::Extern => Extern,
            TT::Var => Var,
            TT::Const => Const,
            TT::Struct => Struct,
            TT::Enum => Enum,
            TT::Return => Return,
            TT::If => If,
            TT::Else => Else,
            TT::While => While,
            TT::For => For,
            TT::Pub => Pub,
            TT::Async => Async,
            TT::Await => Await,
            TT::Match => Match,
            TT::Impl => Impl,
            TT::True => True,
            TT::False => False,
            TT::Undefined => Undefined,
            TT::Break => Break,
            TT::Continue => Continue,
            TT::Package => Package,
            TT::Import => Import,
            TT::As => As,

            TT::Ident(_) => Ident,

            TT::EOF => EOF,
        }
    }
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
                Dot => ".",

                IntLit => "integer literal",
                FloatLit => "float literal",
                StrLit => "string literal",
                CharLit => "char literal",

                Fn => "keyword `fn`",
                Extern => "keyword `extern`",
                Var => "keyword `var`",
                Const => "keyword `const`",
                Struct => "keyword `struct`",
                Enum => "keyword `enum`",
                Return => "keyword `return`",
                If => "keyword `if`",
                Else => "keyword `else`",
                While => "keyword `while`",
                For => "keyword `for`",
                Pub => "keyword `pub`",
                Async => "keyword `async`",
                Await => "keyword `await`",
                Match => "keyword `match`",
                Impl => "keyword `impl`",
                True => "keyword `true`",
                False => "keyword `false`",
                Undefined => "keyword `undefined`",
                Break => "keyword `break`",
                Continue => "keyword `continue`",
                Package => "keyword `package`",
                Import => "keyword `import`",
                As => "keyword `as`",

                Ident => "identifier",

                EOF => "end of file",
            }
        )
    }
}

pub type CodeSpan = Range<usize>;

#[derive(Debug, Clone)]
pub struct LogContext<'a> {
    file: &'a str,
    file_path: &'a Path,
    logs: Vec<BuiltLog>,
    color: ColorChoice,
}

impl<'a> LogContext<'a> {
    pub fn new(file: &'a str, file_path: &'a Path, color: ColorChoice) -> LogContext<'a> {
        LogContext {
            file,
            file_path,
            logs: Vec::new(),
            color,
        }
    }

    pub fn with_stream(
        file: &'a str,
        file_path: &'a Path,
        color: ColorChoice,
        stream: LogStream,
    ) -> LogContext<'a> {
        LogContext {
            file,
            file_path,
            logs: stream.logs,
            color,
        }
    }

    /// Add a BuildLog to the error stream
    pub fn push_built(&mut self, blog: BuiltLog) {
        self.logs.push(blog);
    }

    pub fn push_boxed(&mut self, boxed_log: Box<dyn Log>) {
        self.logs.push(boxed_log.build(self));
    }

    /// Returns true if their is at least one `Log` with `LogLevel` of `Error`, instead false.
    pub fn failed(&self) -> bool {
        for log in &self.logs {
            if let LogLevel::Error = log.lvl {
                return true;
            }
        }
        false
    }

    /// Gives the line and column in the file based on a given index.
    pub fn line_col(&self, index: usize) -> CodeLocation {
        let mut line = 1;
        let mut col = 1;

        for (idx, ch) in self.file.char_indices() {
            if index == idx {
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

        CodeLocation { col, line }
    }

    /// Get the line content with a given `CodeLocation`
    pub fn get_line(&self, loc: CodeLocation) -> String {
        self.file.lines().nth(loc.line - 1).unwrap().to_string()
    }

    /// Build a `Log` into a `BuiltLog`
    pub fn build<L: Log>(&self, log: L) -> BuiltLog {
        log.build(self)
    }

    /// Build a `Log` and returns it into a box
    pub fn build_boxed<L: Log>(&self, log: L) -> Box<BuiltLog> {
        Box::new(self.build(log))
    }

    pub fn push<L: Log>(&mut self, log: L) {
        self.logs.push(self.build(log))
    }

    pub fn push_many(&mut self, logs: Vec<BuiltLog>) {
        self.logs.extend(logs);
    }

    pub fn print(&self) {
        let mut stdout = StandardStream::stdout(self.color);
        self.format(&mut stdout).expect("error formating failed.");
    }

    pub fn format(&self, s: &mut StandardStream) -> Result<(), io::Error> {
        self.stream().format(s)
    }

    pub fn stream(&self) -> LogStream {
        LogStream {
            logs: self.logs.clone(),
            color: self.color,
        }
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
        let location = self.location();
        let start = ctx.line_col(location.start);
        let end = ctx.line_col(location.end);
        assert_eq!(
            start.line, end.line,
            "Doesn't yet support multiple line errors"
        );
        BuiltLog {
            file_path: ctx.file_path.to_path_buf(),
            loc: start.clone(),
            lvl: self.level(),
            msg: self.msg(),
            note: self.note(),
            code: ctx.get_line(start.clone()),
            span: start.col..end.col,
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
        writeln!(s)?;
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

pub enum FinalRes<'a, T> {
    Ok(T, LogContext<'a>),
    Err(LogStream),
}

pub struct LogStream {
    logs: Vec<BuiltLog>,
    color: ColorChoice,
}

impl LogStream {
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
        s.reset()?;
        s.flush()?;
        Ok(())
    }
}
