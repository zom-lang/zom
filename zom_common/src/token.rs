//! This is the token of Zom
//!
//! It is in its own file because later on, there will be lot of tokens type.

use std::fmt;

pub use Token::*;

/// Plus, `+`
pub const OP_PLUS: &str = "+";
/// Minus, `-`
pub const OP_MINUS: &str = "-";
/// Mul, `+`
pub const OP_MUL: &str = "*";
/// Div, `/`
pub const OP_DIV: &str = "/";
/// Mod (remainder), `%`
pub const OP_MOD: &str = "%";
/// Power, `^`
pub const OP_POWER: &str = "^";

/// Equal, `=`, used to assignement
pub const OP_EQ: &str = "=";

/// Compare Equality, `==`
pub const OP_COMP_EQ: &str = "==";
/// Compare Non-Equality, `!=`
pub const OP_COMP_NE: &str = "!=";
/// Compare Greater than, `>`
pub const OP_COMP_GT: &str = ">";
/// Compare Less than, `<`
pub const OP_COMP_LT: &str = "<";
/// Compare Greater Than or Equal to, `=>`
pub const OP_COMP_GTE: &str = "=>";
/// Compare Less than or Equal to, `=<`
pub const OP_COMP_LTE: &str = "=<";

/// Logical OR, `||`
pub const OP_OR: &str = "||";
/// Logical AND, `&&`
pub const OP_AND: &str = "&&";

pub const OP_MAX_LENGHT: usize = 2;

/// Operator Precedence Value for Mul, Div and MOD
pub const PRECEDE_MUL_DIV_MOD: i32 = 60;

/// Operator Precedence Value for ADD and SUB
pub const PRECEDE_ADD_SUB: i32 = 40;

/// Operator Precedence Value for COMP_LT, COMP_GT, COMP_LTE and COMP_GTE
pub const PRECEDE_COMP: i32 = 20;

/// Operator Precedence Value for COMPE_EQ and COMP_NE
pub const PRECEDE_EQ_NE: i32 = 10;

/// Operator Precedence Value for AND
pub const PRECEDE_AND: i32 = 6;

/// Operator Precedence Value for OR
pub const PRECEDE_OR: i32 = 5;

/// Operator Precedence Value for EQ (assignement)
pub const PRECEDE_EQ: i32 = 2;

/// This function get the first char of a potentil operator
pub fn is_start_operator(maybe_start: char) -> bool {
    let maybe_start = maybe_start.to_string();

    OP_PLUS.starts_with(maybe_start.as_str())
        || OP_MINUS.starts_with(maybe_start.as_str())
        || OP_MUL.starts_with(maybe_start.as_str())
        || OP_DIV.starts_with(maybe_start.as_str())
        || OP_MOD.starts_with(maybe_start.as_str())
        || OP_POWER.starts_with(maybe_start.as_str())
        || OP_EQ.starts_with(maybe_start.as_str())
        || OP_COMP_EQ.starts_with(maybe_start.as_str())
        || OP_COMP_NE.starts_with(maybe_start.as_str())
        || OP_COMP_GT.starts_with(maybe_start.as_str())
        || OP_COMP_LT.starts_with(maybe_start.as_str())
        || OP_COMP_GTE.starts_with(maybe_start.as_str())
        || OP_COMP_LTE.starts_with(maybe_start.as_str())
        || OP_OR.starts_with(maybe_start.as_str())
        || OP_AND.starts_with(maybe_start.as_str())
}

/// Check if the given string slice is an Operator (OP_**)
///
/// return a tuple, the first element is if it's an operator and the second is the lenght of the operator.
pub fn is_operator(maybe_op: &str) -> (bool, usize) {
    // I think it can be improved...
    // Single char operator.
    if maybe_op.starts_with(OP_PLUS)
        || maybe_op.starts_with(OP_MINUS)
        || maybe_op.starts_with(OP_MUL)
        || maybe_op.starts_with(OP_DIV)
        || maybe_op.starts_with(OP_MOD)
        || maybe_op.starts_with(OP_POWER)
        || maybe_op.starts_with(OP_COMP_GT)
        || maybe_op.starts_with(OP_COMP_LT)
    {
        (true, 1)
    } else if maybe_op.starts_with(OP_EQ) {
        match maybe_op.get(1..=1) {
            Some("=") | Some("<") | Some(">") => {
                return (true, 2);
            }
            _ => (),
        }

        (true, 1)
    }
    // Dual char operator.
    else if maybe_op == OP_COMP_NE || maybe_op == OP_OR || maybe_op == OP_AND {
        (true, 2)
    }
    // it's not an OP_**
    else {
        (false, 0)
    }
}

/// const for the keyword `func`
pub const KEY_FUNC: &str = "func";
/// const for the keyword `extern`
pub const KEY_EXTERN: &str = "extern";
/// const for the keyword `let`
pub const KEY_LET: &str = "let";

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Operators
    /// Operators, should only be an OP_**
    Operator(String),

    // Structural symbols
    OpenParen,  // ` ( `
    CloseParen, // ` ) `

    OpenBracket,  // ` [ `
    CloseBracket, // ` ] `

    OpenBrace,  // ` { `
    CloseBrace, // ` } `

    Colon,     // ` : `
    SemiColon, // ` ; `
    Comma,     // ` , `

    // Literals
    Int(i32),
    Float(f32),

    // Keywords
    Func,
    Extern,
    Let,

    // Identifier
    Ident(String), // Identifier is a alphanumeric string
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Token::Int(val) = &self {
            return write!(f, "Int:{}", val);
        } else if let Token::Float(val) = &self {
            return write!(f, "Float:{}", val);
        }
        write!(f, "{:?}", &self)
    }
}
