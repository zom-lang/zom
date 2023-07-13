//! This module contains parsing for statements.

use super::{expr::Expression, types::Type};

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Var {
        name: String,
        type_: Option<Type>,
        expr: Expression,
    },
    Const {
        name: String,
        type_: Option<Type>,
        expr: Expression,
    },
    Return,
}
