//! This is the token of Zom
//!
//! It is in its own file because later on, there will be lot of tokens type.

use std::{
    fmt::{self, Display},
    ops::RangeInclusive,
};

pub use TokenType::*;

/// Mul, `*`
pub const OP_MUL: &str = "*";
/// Div, `/`
pub const OP_DIV: &str = "/";
/// Remainder, `%`
pub const OP_REM: &str = "%";
/// Plus, `+`
pub const OP_ADD: &str = "+";
/// Minus, `-`
pub const OP_SUB: &str = "-";
/// Right shift, `>>`
pub const OP_RSHIFT: &str = ">>";
/// Left shift, `<<`
pub const OP_LSHIFT: &str = "<<";

/// Compare Less than, `<`
pub const OP_COMP_LT: &str = "<";
/// Compare Greater than, `>`
pub const OP_COMP_GT: &str = ">";
/// Compare Less than or Equal to, `<=`
pub const OP_COMP_LTE: &str = "<=";
/// Compare Greater Than or Equal to, `>=`
pub const OP_COMP_GTE: &str = ">=";
/// Compare Equality, `==`
pub const OP_COMP_EQ: &str = "==";
/// Compare Non-Equality, `!=`
pub const OP_COMP_NE: &str = "!=";

/// Bitwise AND, `&`
pub const OP_BIT_AND: &str = "&";
/// Bitwise XOR, `&`
pub const OP_BIT_XOR: &str = "^";
/// Bitwise OR, `|`
pub const OP_BIT_OR: &str = "|";
/// Bitwise NOT, `~`
pub const OP_BIT_NOT: &str = "~";

/// Logical AND, `&&`
pub const OP_LOGIC_AND: &str = "&&";
/// Logical OR, `||`
pub const OP_LOGIC_OR: &str = "||";
/// Logical NOT, `!`
pub const OP_LOGIC_NOT: &str = "!";

/// Simple assignement, `=`,
pub const OP_EQ: &str = "=";

/// Borrow, `&` can be followed by the keyword `var`
/// for var borrow
pub const OP_BORROW: &str = "&";
/// Dereferencing, `*`
pub const OP_DEREF: &str = "*";

/// Minus, `-` (unary)
pub const OP_MINUS: &str = "-";
/// Plus, `+` (unary)
pub const OP_PLUS: &str = "+";



/// Operator Precedence Value for Mul, Div and MOD
pub const PRECEDE_MUL_DIV_REM: i32 = 60;

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
        || OP_REM.starts_with(maybe_start.as_str())
        || OP_EQ.starts_with(maybe_start.as_str())
        || OP_COMP_EQ.starts_with(maybe_start.as_str())
        || OP_COMP_NE.starts_with(maybe_start.as_str())
        || OP_COMP_GT.starts_with(maybe_start.as_str())
        || OP_COMP_LT.starts_with(maybe_start.as_str())
        || OP_COMP_GTE.starts_with(maybe_start.as_str())
        || OP_COMP_LTE.starts_with(maybe_start.as_str())
        // || OP_OR.starts_with(maybe_start.as_str())
        // || OP_AND.starts_with(maybe_start.as_str())
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
        || maybe_op.starts_with(OP_REM)
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
    else if maybe_op == OP_COMP_NE
    // || maybe_op == OP_OR || maybe_op == OP_AND
    {
        (true, 2)
    }
    // it's not an OP_**
    else {
        (false, 0)
    }
}

/// const for the keyword `func`
pub const KW_FUNC: &str = "func";

/// const for the keyword `extern`
pub const KW_EXTERN: &str = "extern";

/// const for the keyword `let`
pub const KW_VAR: &str = "var";

/// const for the keyword `const`
pub const KW_CONST: &str = "const";

/// const for the keyword `struct`
pub const KW_STRUCT: &str = "struct";

/// const for the keyword `enum`
pub const KW_ENUM: &str = "enum";

/// const for the keyword `enum`
pub const KW_RETURN: &str = "return";

/// const for the keyword `if`
pub const KW_IF: &str = "if";

/// const for the keyword `else`
pub const KW_ELSE: &str = "else";

/// const for the keyword `while`
pub const KW_WHILE: &str = "while";

/// const for the keyword `for`
pub const KW_FOR: &str = "for";

/// const for the keyword `pub`
pub const KW_PUB: &str = "pub";

/// const for the keyword `async`
pub const KW_ASYNC: &str = "async";

/// const for the keyword `await`
pub const KW_AWAIT: &str = "await";

/// const for the keyword `match`
pub const KW_MATCH: &str = "match";

/// const for the keyword `impl`
pub const KW_IMPL: &str = "impl";

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// `tt` means token type.
    pub tt: TokenType,
    pub span: RangeInclusive<usize>,
}

impl Token {
    pub fn new(tt: TokenType, span: RangeInclusive<usize>) -> Token {
        Token { tt, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Operators
    /// Operators, should only be an OP_** constant.
    Operator(String),

    // Structural symbols

    // in the Zom self compiling compiler change it to :
    OpenParen,  // ` ( ` LParen
    CloseParen, // ` ) `RParen

    OpenBracket,  // ` [ ` LBracket
    CloseBracket, // ` ] ` RBracket

    OpenBrace,  // ` { ` LBrace
    CloseBrace, // ` } ` RBrace

    Colon,     // ` : `
    SemiColon, // ` ; `
    Comma,     // ` , `
    At,        // ` @ `

    // Literals
    Int(i32),
    Float(f32),
    Str(String),
    Char(char),

    // Keywords
    Func,
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

    // Identifier
    Ident(String), // Identifier is a alphanumeric with `_` string
    Lifetime(String),

    EOF,
}
impl TokenType {
    pub fn format_toks(tokens: Vec<TokenType>) -> String {
        let mut s = "".to_owned();
        for (len, tt) in tokens.iter().enumerate() {
            if len == tokens.len() - 2 {
                s += format!("{} ", tt).as_str();
                continue;
            } else if len == tokens.len() - 1 {
                s += format!("or {}", tt).as_str();
                continue;
            }
            s += format!("{}, ", tt).as_str();
        }
        s
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator(op) => write!(f, "operator `{}`", op),
            OpenParen => write!(f, "`(`"),
            CloseParen => write!(f, "`)`"),
            OpenBracket => write!(f, "`[`"),
            CloseBracket => write!(f, "`]`"),
            OpenBrace => write!(f, "`{{`"),
            CloseBrace => write!(f, "`}}`"),

            Colon => write!(f, "`:`"),
            SemiColon => write!(f, "`;`"),
            Comma => write!(f, "`,`"),
            At => write!(f, "`@`"),

            Int(_) => write!(f, "integer literral"),
            Float(_) => write!(f, "float literral"),
            Str(_) => write!(f, "string literral"),
            Char(_) => write!(f, "char literral"),

            Func => write!(f, "keyword `func`"),
            Extern => write!(f, "keyword `extern`"),
            Var => write!(f, "keyword `var`"),
            Const => write!(f, "keyword `const`"),
            Struct => write!(f, "keyword `struct`"),
            Enum => write!(f, "keyword `enum`"),
            Return => write!(f, "keyword `return`"),
            If => write!(f, "keyword `if`"),
            Else => write!(f, "keyword `else`"),
            While => write!(f, "keyword `while`"),
            For => write!(f, "keyword `for`"),
            Pub => write!(f, "keyword `pub`"),
            Async => write!(f, "keyword `async`"),
            Await => write!(f, "keyword `await`"),
            Match => write!(f, "keyword `match`"),
            Impl => write!(f, "keyword `impl`"),

            Ident(_) => write!(f, "identifier"),
            Lifetime(_) => write!(f, "lifetime"),

            EOF => write!(f, "End of File"),
        }
    }
}
