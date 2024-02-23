//! This is the token of Zom
//!
//! It is in its own file because later on, there will be lot of tokens type.

use std::{
    fmt::{self, Display},
    ops::Range,
    str::FromStr,
};

pub use TokenType::*;

/// Ampersand, `&`
pub const OP_AMPERSAND: &str = "&";
/// Asterisk, `*`
pub const OP_ASTERISK: &str = "*";
/// Caret, `^`
pub const OP_CARET: &str = "^";
/// Dot, `.`
pub const OP_DOT: &str = ".";
/// DotAsterisk, `.*`
pub const OP_DOTASTERISK: &str = ".*";
/// Equal, `=`,
pub const OP_EQUAL: &str = "=";
/// Equal2, `==`
pub const OP_EQUAL2: &str = "==";
/// Exclamationmark, `!`
pub const OP_EXCLAMATIONMARK: &str = "!";
/// ExclamationmarkEqual, `!=`
pub const OP_EXCLAMATIONMARKEQUAL: &str = "!=";
/// LArrow, `<`
pub const OP_LARROW: &str = "<";
/// LArrow2, `<<`
pub const OP_LARROW2: &str = "<<";
/// LArrowEqual, `<=`
pub const OP_LARROWEQUAL: &str = "<=";
/// Minus, `-`
pub const OP_MINUS: &str = "-";
/// Percent, `%`
pub const OP_PERCENT: &str = "%";
/// Pipe2, `||`
pub const OP_PIPE2: &str = "||";
/// Plus, `+`
pub const OP_PLUS: &str = "+";
/// RArrow, `>`
pub const OP_RARROW: &str = ">";
/// RArrow2, `>>`
pub const OP_RARROW2: &str = ">>";
/// RArrowEqual, `>=`
pub const OP_RARROWEQUAL: &str = ">=";
/// Slash, `/`
pub const OP_SLASH: &str = "/";

/// Maximum operator lenght
pub const OPERATOR_LENGHT: usize = 2;

/// List of unique operators (contains no aliases)
pub const OPERATORS: [&str; 20] = [
    OP_AMPERSAND,
    OP_ASTERISK,
    OP_CARET,
    OP_DOT,
    OP_DOTASTERISK,
    OP_EQUAL,
    OP_EQUAL2,
    OP_EXCLAMATIONMARK,
    OP_EXCLAMATIONMARKEQUAL,
    OP_LARROW,
    OP_LARROW2,
    OP_LARROWEQUAL,
    OP_MINUS,
    OP_PERCENT,
    OP_PIPE2,
    OP_PLUS,
    OP_RARROW,
    OP_RARROW2,
    OP_RARROWEQUAL,
    OP_SLASH,
];

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Operator {
    Ampersand,
    Asterisk,
    Caret,
    Dot,
    DotAsterisk,
    Equal,
    Equal2,
    Exclamationmark,
    ExclamationmarkEqual,
    LArrow,
    LArrow2,
    LArrowEqual,
    Minus,
    Percent,
    Pipe2,
    Plus,
    RArrow,
    RArrow2,
    RArrowEqual,
    Slash,
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Operator::*;
        let op = match *self {
            Ampersand => OP_AMPERSAND,
            Asterisk => OP_ASTERISK,
            Caret => OP_CARET,
            Dot => OP_DOT,
            DotAsterisk => OP_DOTASTERISK,
            Equal => OP_EQUAL,
            Equal2 => OP_EQUAL2,
            Exclamationmark => OP_EXCLAMATIONMARK,
            ExclamationmarkEqual => OP_EXCLAMATIONMARKEQUAL,
            LArrow => OP_LARROW,
            LArrow2 => OP_LARROW2,
            LArrowEqual => OP_LARROWEQUAL,
            Minus => OP_MINUS,
            Percent => OP_PERCENT,
            Pipe2 => OP_PIPE2,
            Plus => OP_PLUS,
            RArrow => OP_RARROW,
            RArrow2 => OP_RARROW2,
            RArrowEqual => OP_RARROWEQUAL,
            Slash => OP_SLASH,
        };
        write!(f, "{op}")
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Operator::*;
        Ok(match s {
            OP_AMPERSAND => Ampersand,
            OP_ASTERISK => Asterisk,
            OP_CARET => Caret,
            OP_DOT => Dot,
            OP_EQUAL => Equal,
            OP_EQUAL2 => Equal2,
            OP_EXCLAMATIONMARK => Exclamationmark,
            OP_EXCLAMATIONMARKEQUAL => ExclamationmarkEqual,
            OP_LARROW => LArrow,
            OP_LARROW2 => LArrow2,
            OP_LARROWEQUAL => LArrowEqual,
            OP_MINUS => Minus,
            OP_PERCENT => Percent,
            OP_PIPE2 => Pipe2,
            OP_PLUS => Plus,
            OP_RARROW => RArrow,
            OP_RARROW2 => RArrow2,
            OP_RARROWEQUAL => RArrowEqual,
            OP_SLASH => Slash,
            op => return Err(format!("unknown binary operator `{}`", op)),
        })
    }
}

/// Operator Precedence Value for Mul Div Rem
pub const PR_MUL_DIV_REM: u16 = 7;
/// Operator Precedence Value for Add Sub
pub const PR_ADD_SUB: u16 = 6;
/// Operator Precedence Value for Right and Left shifts
pub const PR_SHIFT: u16 = 5;
/// Operator Precedence Value for Less than, Greater than, Less than or equal to and greater than or equal to
pub const PR_COMP: u16 = 4;
/// Operator Precedence Value for Eq Ne
pub const PR_COMP_EQ_NE: u16 = 3;
/// Operator Precedence Value for And
pub const PR_AND: u16 = 2;
/// Operator Precedence Value for Xor
pub const PR_XOR: u16 = 1;
/// Operator Precedence Value for Or
pub const PR_OR: u16 = 0;

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
            Oper(op) => write!(f, "`{}`", op),
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

            Ident(_) => write!(f, "identifier"),

            EOF => write!(f, "End of File"),
        }
    }
}
