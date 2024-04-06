//! This is the token of Zom
//!
//! It is in its own file because later on, there will be lot of tokens type.

use std::{
    fmt::{self, Display},
    ops::Range,
};

pub use TokenType::*;

use crate::operator::Operator;

// KEYWORDS

/// const for the keyword `fn`
pub const KW_FN: &str = "fn";

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

/// const for the keyword `break`
pub const KW_BREAK: &str = "break";

/// const for the keyword `continue`
pub const KW_CONTINUE: &str = "continue";

/// const for the keyword `package`
pub const KW_PACKAGE: &str = "package";

/// const for the keyword `import`
pub const KW_IMPORT: &str = "import";

/// const for the keyword `as`
pub const KW_AS: &str = "as";

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
    Oper(Operator),

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
    Int(u64),
    Float(f32),
    Str(String),
    Char(char),

    // Keywords
    Fn,
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
    Break,
    Continue,
    Package,
    Import,
    As,

    // Identifier
    Ident(String), // Identifier is a alphanumeric with `_` string

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
            Oper(op) => write!(f, "operator `{}`", op),
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

            Fn => write!(f, "keyword `fn`"),
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
            Break => write!(f, "keyword `break`"),
            Continue => write!(f, "keyword `continue`"),
            Package => write!(f, "keyword `package`"),
            Import => write!(f, "keyword `import`"),
            As => write!(f, "keyword `as`"),

            Ident(name) => write!(f, "identifier {name}"),

            EOF => write!(f, "End of File"),
        }
    }
}
