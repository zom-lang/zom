use std::fmt;
use std::{ops::Range, path::PathBuf};

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

pub struct LogContext {
    file: String,
    file_path: PathBuf,
    logs: Vec<BuiltLog>,
}

impl LogContext {
    pub fn new(file: String, file_path: PathBuf) -> LogContext {
        LogContext {
            file,
            file_path,
            logs: Vec::new(),
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
                BuiltLog {
                    file_path: self.file_path.clone(),
                    loc: loc.clone(),
                    lvl: LogLevel::Error,
                    msg: format!("expected {}, found {}", format_tokens(&expected), found),
                    code: self.get_line(loc),
                    span: location,
                }
            }
            _ => todo!("Not implemented"),
        }
    }

    pub fn add(&mut self, log: Log) {
        self.logs.push(self.build_log(log))
    }

    pub fn print(&self) {
        // Modify this function to take a writer or smth like that
        for log in &self.logs {
            println!("{log}")
        }
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

#[derive(Clone)]
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

pub fn spaces(n: usize) -> String {
    let mut s = String::with_capacity(n);
    for _ in 0..n {
        s.push(' ');
    }
    s
}

pub struct BuiltLog {
    file_path: PathBuf,
    loc: CodeLocation,
    lvl: LogLevel,
    msg: String,
    code: String,
    span: CodeSpan,
}

impl fmt::Display for BuiltLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {}: {}\n",
            self.file_path.clone().into_os_string().to_str().unwrap(),
            self.loc.line,
            self.loc.col,
            self.lvl,
            self.msg
        )?;
        write!(f, "{}\n", self.code)?;
        write!(f, "{}^", spaces(self.loc.col - 1))?;
        fmt::Result::Ok(())
    }
}

pub enum LogLevel {
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}
