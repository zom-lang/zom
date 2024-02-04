use zom_errors::prelude::*;

/// unknown token lexing error
pub struct UnknownToken {
    /// character found
    pub found: char,
    /// location of the found token
    pub location: CodeSpan,
}

impl Log for UnknownToken {
    fn location(&self) -> CodeSpan {
        self.location.clone()
    }

    fn level(&self) -> LogLevel {
        LogLevel::Error
    }

    fn msg(&self) -> String {
        format!("unknown start of token, '{}'", self.found)
    }
}

/// unknown escape sequence in string or char literal
pub struct UnknownEscape {
    /// character found
    pub escape: char,
    /// did the escape came from a string or a char literal
    pub is_string: bool,
    /// location of the found token
    pub location: CodeSpan,
}

impl Log for UnknownEscape {
    fn location(&self) -> CodeSpan {
        self.location.clone()
    }

    fn level(&self) -> LogLevel {
        LogLevel::Error
    }

    fn msg(&self) -> String {
        format!("unknown character escape: '{}'", self.escape)
    }

    fn note(&self) -> Option<String> {
        Some(
            r"supported escapse sequence are, '\0', '\n', '\r', '\t', '\xNN' (not yet supported) "
                .to_string()
                + if self.is_string {
                    r#"and '\"'."#
                } else {
                    r"and '\''"
                },
        )
    }
}

/// unknown token lexing error
pub struct UnterminatedQuoteLit {
    /// character found
    pub is_char: bool,
    /// location of the found token
    pub location: CodeSpan,
}

impl Log for UnterminatedQuoteLit {
    fn location(&self) -> CodeSpan {
        self.location.clone()
    }

    fn level(&self) -> LogLevel {
        LogLevel::Error
    }

    fn msg(&self) -> String {
        if self.is_char {
            "unterminated char literal"
        } else {
            "unterminated string literal"
        }
        .to_string()
    }
}
