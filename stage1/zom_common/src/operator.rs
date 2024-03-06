use std::fmt::{self, Display};
use std::str::FromStr;

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

/// Operator Precedence Value for Unary Dereference
pub const PR_DEREFERENCE: u16 = 9;
/// Operator Precedence Value for Unary Operations: AddressOf, Negation, Not, but not Dereference
pub const PR_UNARY: u16 = 8;
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
