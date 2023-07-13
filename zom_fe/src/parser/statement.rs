//! This module contains parsing for statements.

use super::{expr::Expression, Type};

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
