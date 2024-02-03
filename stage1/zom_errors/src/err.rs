use crate::prelude::*;

/// A simple error struct, please use it only when making a custom error
/// structure is irrelevant. Use instead custom error structs.
pub struct SimpleLog {
    level: LogLevel,
    msg: String,
    note: Option<String>,
    location: CodeSpan,
}

impl Log for SimpleLog {
    fn location(&self) -> CodeSpan {
        self.location.clone()
    }

    fn level(&self) -> LogLevel {
        self.level.clone()
    }

    fn note(&self) -> Option<String> {
        self.note.clone()
    }

    fn msg(&self) -> String {
        self.msg.clone()
    }
}
