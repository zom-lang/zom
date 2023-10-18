//! This is the token of Zom
//!
//! It is in its own file because later on, there will be lot of tokens type.

use std::{
    fmt::{self, Display},
    ops::Range,
    str::FromStr,
};

pub use TokenType::*;

/// Multiply, `*`
pub const OP_MUL: &str = "*";
/// Divide, `/`
pub const OP_DIV: &str = "/";
/// Remainder, `%`
pub const OP_REM: &str = "%";
/// Addition, `+`
pub const OP_ADD: &str = "+";
/// Subtraction, `-`
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

/// XOR, `^`
pub const OP_XOR: &str = "^";
/// AND, `&&`
pub const OP_AND: &str = "&&";
/// OR, `||`
pub const OP_OR: &str = "||";
/// NOT, `!`
pub const OP_NOT: &str = "!";

/// Address Of, `&`
pub const OP_ADDR_OF: &str = "&";

/// Simple assignement, `=`,
pub const OP_EQ: &str = "=";

pub const OP_MAX_LENGHT: usize = 2;

/// List of unique operators (contains no aliases)
pub const OPERATORS: [&str; 19] = [
    OP_MUL,
    OP_DIV,
    OP_REM,
    OP_ADD,
    OP_SUB,
    OP_RSHIFT,
    OP_LSHIFT,
    OP_COMP_LT,
    OP_COMP_GT,
    OP_COMP_LTE,
    OP_COMP_GTE,
    OP_COMP_EQ,
    OP_COMP_NE,
    OP_AND,
    OP_OR,
    OP_XOR,
    OP_NOT,
    OP_ADDR_OF,
    OP_EQ,
];

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Operator {
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    RShift,
    LShift,
    CompLT,
    CompGT,
    CompLTE,
    CompGTE,
    CompEq,
    CompNe,
    And,
    Or,
    Xor,
    Not,
    AddrOf,
    Equal,
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Operator::*;
        let op = match *self {
            Mul => OP_MUL,
            Div => OP_DIV,
            Rem => OP_REM,
            Add => OP_ADD,
            Sub => OP_SUB,
            RShift => OP_RSHIFT,
            LShift => OP_LSHIFT,

            CompLT => OP_COMP_LT,
            CompGT => OP_COMP_GT,
            CompLTE => OP_COMP_LTE,
            CompGTE => OP_COMP_GTE,
            CompEq => OP_COMP_EQ,
            CompNe => OP_COMP_NE,

            And => OP_AND,
            Or => OP_OR,
            Xor => OP_XOR,
            Not => OP_NOT,

            AddrOf => OP_ADDR_OF,

            Equal => OP_EQ,
        };
        write!(f, "{}", op)
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Operator::*;
        match s {
            OP_MUL => Ok(Mul),
            OP_DIV => Ok(Div),
            OP_REM => Ok(Rem),
            OP_ADD => Ok(Add),
            OP_SUB => Ok(Sub),
            OP_RSHIFT => Ok(RShift),
            OP_LSHIFT => Ok(LShift),

            OP_COMP_LT => Ok(CompLT),
            OP_COMP_GT => Ok(CompGT),
            OP_COMP_LTE => Ok(CompLTE),
            OP_COMP_GTE => Ok(CompGTE),
            OP_COMP_EQ => Ok(CompEq),
            OP_COMP_NE => Ok(CompNe),

            OP_AND => Ok(And),
            OP_OR => Ok(Or),
            OP_XOR => Ok(Xor),
            OP_NOT => Ok(Not),

            OP_EQ => Ok(Equal),
            op => Err(format!("unknown binary operator `{}`", op)),
        }
    }
}

/// Operator Precedence Value for Unary operators
pub const PR_UNARY: i32 = 12;
/// Operator Precedence Value for Mul Div Rem
pub const PR_MUL_DIV_REM: i32 = 11;
/// Operator Precedence Value for Add Sub
pub const PR_ADD_SUB: i32 = 10;
/// Operator Precedence Value for Right and Left shifts
pub const PR_SHIFT: i32 = 9;
/// Operator Precedence Value for Less than, Greater than, Less than or equal to and greater than or equal to
pub const PR_COMP: i32 = 8;
/// Operator Precedence Value for Eq Ne
pub const PR_COMP_EQ_NE: i32 = 7;
/// Operator Precedence Value for Xor
pub const PR_XOR: i32 = 5;
/// Operator Precedence Value for And
pub const PR_AND: i32 = 3;
/// Operator Precedence Value for Or
pub const PR_OR: i32 = 2;
/// Operator Precedence Value for Equal
pub const PR_EQ: i32 = 1;

// KEYWORDS

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

/// const for the keyword `true`
pub const KW_TRUE: &str = "true";

/// const for the keyword `false`
pub const KW_FALSE: &str = "false";

/// const for the keyword `undefined`
pub const KW_UNDEFINED: &str = "undefined";

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// `tt` means token type.
    pub tt: TokenType,
    pub span: Range<usize>,
}

impl Token {
    pub fn new(tt: TokenType, span: Range<usize>) -> Token {
        Token { tt, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Operators
    /// Operators, should only be an OP_** constant.
    Operator(Operator),

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
    Dot,       // ` . `

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
    True,
    False,
    Undefined,

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
            Operator(op) => write!(f, "`{}`", op),
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
            Dot => write!(f, "`.`"),

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
            True => write!(f, "keyword `true`"),
            False => write!(f, "keyword `false`"),
            Undefined => write!(f, "keyword `undefined`"),

            Ident(_) => write!(f, "identifier"),
            Lifetime(_) => write!(f, "lifetime"),

            EOF => write!(f, "End of File"),
        }
    }
}
