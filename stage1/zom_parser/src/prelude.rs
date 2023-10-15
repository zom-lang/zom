//! Private prelude for the parser, used to remove repetitive code.

pub use std::ops::Range;

pub use zom_common::token::{Token, TokenType::*, *};

pub use crate::{
    err_et, expect_token, impl_span, parse_try, token_parteq, ParserSettings, ParsingContext,
    PartParsingResult::{self, *},
};

pub use zom_common::error::Position;
pub use zom_common::error::ZomError;
