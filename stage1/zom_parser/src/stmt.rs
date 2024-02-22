//! Module responsible for parsing statement.
use crate::prelude::*;

#[derive(Debug)]
pub struct Statement {
    pub span: Range<usize>,
}
