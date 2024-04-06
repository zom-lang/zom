use crate::prelude::*;

/// A simple error struct, please use it only when making a custom error
/// structure is irrelevant. Use instead custom error structs.
pub struct SimpleLog {
    pub level: LogLevel,
    pub msg: Box<str>,
    pub cursor_msg: Option<Box<str>>,
    pub location: CodeSpan,
}

impl Log for SimpleLog {
    fn location(&self) -> CodeSpan {
        self.location.clone()
    }

    fn level(&self) -> LogLevel {
        self.level.clone()
    }

    fn cursor_msg(&self) -> Option<Box<str>> {
        self.cursor_msg.clone()
    }

    fn msg(&self) -> Box<str> {
        self.msg.clone()
    }
}

/// Unexpected End of file while parsing or lexing.
pub struct UnexpectedEOF(pub CodeSpan);

impl Log for UnexpectedEOF {
    fn location(&self) -> CodeSpan {
        self.0.clone()
    }

    fn level(&self) -> LogLevel {
        LogLevel::Error
    }

    fn msg(&self) -> Box<str> {
        "unexpected end of file".into()
    }
}
