//! The module containing the lexer.
use std::ops::Range;
use std::path::Path;

use zom_errors::prelude::*;

use zom_common::operator::Operator;
use zom_common::token::{Token, TokenType, TokenType::*};

use zom_common::token::*;

pub mod err;

/// This is a struct representing a Zom source file, it contains a ref to both the path and the text
#[derive(Debug)]
pub struct ZomFile<'a> {
    path: &'a Path,
    text: &'a str,
}

impl<'a> ZomFile<'a> {
    /// Create a new ZomFile
    pub fn new(text: &'a str, path: &'a Path) -> Self {
        Self { path, text }
    }

    /// Get the text contained inside the ZomFile
    pub fn text(&self) -> &str {
        self.text
    }

    /// Get the path contained inside the ZomFile
    pub fn path(&self) -> &Path {
        self.path
    }

    /// Get the nth char inside the file and returns it.
    pub fn get(&self, index: usize) -> Option<char> {
        self.text.chars().nth(index) // the current implementation of this function is very bad, we would need to improve it, e.g: store the iterator instead of recreating it every time.
    }
}

use PartTokenResult::*;

/// Used by lexing methods of the lexer to tell the lexer how the lexing occured.
#[derive(Debug)]
pub enum PartTokenResult {
    /// used when the lexing is entirely successful
    Tok(TokenType),

    /// used when the lexing failed and it cannot lex anymore the content
    ///
    /// In this case, we assume the error doesn't have a position
    Error(BuiltLog),

    /// used when the lexing failed but it can ignore the error and keep lexing
    /// and make result to a TokenType but with multiple errors.
    ///
    /// Using PartSuccess is prefered when possible.
    PartSuccess(TokenType, Vec<BuiltLog>),

    /// used when the lexing result to a comment.
    Comment,

    /// used when the lexing results to a whitespace.
    Whitespace,
}

/// This macro pop a character using the function 'pop()'
/// and assert when the compiler is compiled is debug mode, that
/// the thing poped is what is expected.
macro_rules! pop_expect {
    ($self:expr => $expected:expr $(, $msg:expr)?) => (
        let poped = $self.pop();
        debug_assert_eq!(poped, $expected $(, $msg)?)
    );

    ($self:expr => $expected:expr; $else:stmt) => (
        let poped = $self.pop();
        if !(poped == $expected) {
            $else
        }
    );
}

/// Used to lexe the content of a file into tokens that the parser can understand.
pub struct Lexer<'a> {
    file: ZomFile<'a>,
    prev_idx: usize,
    index: usize,
    pub lctx: LogContext<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, path: &'a Path, lctx: LogContext<'a>) -> Lexer<'a> {
        Lexer {
            file: ZomFile::new(text, path),
            prev_idx: 0,
            index: 0,
            lctx,
        }
    }

    /// Get the ZomFile inside the lexer
    pub fn file(&self) -> &ZomFile {
        &self.file
    }

    /// Get the path of the file.
    pub fn file_path(&self) -> &Path {
        self.file.path()
    }

    /// Get the text of the file.
    pub fn file_text(&self) -> &str {
        self.file.text()
    }

    /// Get the char at the current index, then increment the index by one and returns the char he gets before
    pub fn pop(&mut self) -> Option<char> {
        let c = self.peek();
        self.index += 1;
        c
    }

    /// Get the char at the current index, and returns it. It returns EOF if the index is out of bounds.
    pub fn peek(&self) -> Option<char> {
        self.file.get(self.index)
    }

    /// Get the nth char in the file, at index + offset
    ///
    /// May return `None` if the index is out of bounds of the file text.
    pub fn peek_nth(&self, offset: usize) -> Option<char> {
        self.file.get(self.index + offset)
    }

    /// Lex the whole file and returns either a vector of Tokens if it succeeds or,
    /// a list of errors if it doesn't.
    pub fn lex(&mut self) -> FinalRes<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            self.prev_idx = self.index;
            match self.make_token() {
                Tok(tt) => {
                    if self.push_token(&mut tokens, tt) {
                        // Reached EOF, breaking out of the loop
                        break;
                    }
                }
                Error(err) => {
                    self.lctx.push_built(err);
                }
                PartSuccess(tt, errs) => {
                    debug_assert!(!errs.is_empty());

                    self.lctx.push_many(errs);

                    if self.push_token(&mut tokens, tt) {
                        // Reached EOF, breaking out of the loop
                        break;
                    }
                }
                Comment | Whitespace => {}
            }
        }

        if self.lctx.failed() {
            return FinalRes::Err(self.lctx.stream());
        }
        FinalRes::Ok(tokens, self.lctx.clone())
    }

    fn get_pos(&self) -> Range<usize> {
        self.prev_idx..self.index
    }

    /// This functions takes a vector of tokens, and push a Token containing the given TokenType
    /// with the start arg and the index at that moment.
    ///
    /// And if the TokenType is a EOF it returns true, either false.
    fn push_token(&self, tokens: &mut Vec<Token>, tt: TokenType) -> bool {
        if tt == EOF {
            let text_len = self.file_text().len() - 1;
            // the length of the buffer is used for the span of the EOF, because
            // the EOF is the last char and its 'text_len..text_len' because if for
            // some reason we want to show the EOF in an error we can.
            tokens.push(Token {
                tt,
                span: text_len - 1..text_len,
            });
            return true;
        }
        tokens.push(Token {
            tt,
            span: self.prev_idx..self.index,
        });

        false
    }

    /// Given the current char (self.peek()) calls a function and returns the result.
    fn make_token(&mut self) -> PartTokenResult {
        let t = match self.peek() {
            Some('(') => OpenParen,
            Some(')') => CloseParen,
            Some('[') => OpenBracket,
            Some(']') => CloseBracket,
            Some('{') => OpenBrace,
            Some('}') => CloseBrace,
            Some(';') => SemiColon,
            Some(':') => Colon,
            Some(',') => Comma,
            Some('@') => At,
            Some('/') => {
                self.pop();
                match self.peek() {
                    Some('/') => {
                        self.pop();
                        self.lex_until('\n');
                        return Comment;
                    }
                    _ => return Tok(Oper(Operator::Slash)),
                }
            }
            Some('"') => return self.lex_string_literal(),
            Some('\'') => return self.lex_char_literal(),
            Some('A'..='Z' | 'a'..='z' | '_' | '0'..='9') => return self.lex_word(),
            Some(w) if w.is_whitespace() => {
                self.index += 1;
                return Whitespace;
            }
            Some(c) => {
                if let Some(op) = self.lex_operator() {
                    return Tok(Oper(op));
                }
                self.pop();
                return Error(self.lctx.build(err::UnknownToken {
                    found: c,
                    location: self.get_pos(),
                }));
            }
            None => EOF,
        };
        self.index += 1;

        Tok(t)
    }

    /// Lexes the input until while the content is alphanumeric with underscore(s) returns the content and if the
    /// string is numeric, in a tuple.
    pub fn make_word(&mut self) -> (String, bool) {
        let mut word = String::new();
        let mut is_numeric = true;

        loop {
            match self.peek() {
                Some(c @ ('A'..='Z' | 'a'..='z' | '_')) => {
                    is_numeric = false;
                    word.push(c);
                }
                Some(c @ '0'..='9') => {
                    word.push(c);
                }
                _ => break,
            }
            self.index += 1;
        }
        (word, is_numeric)
    }

    /// Lexes either an integer or an identifier or a keyword, and returns it.
    pub fn lex_word(&mut self) -> PartTokenResult {
        let (word, is_numeric) = self.make_word();

        if is_numeric {
            match self.lex_int(word) {
                Ok(tt) => Tok(tt),
                Err(err) => Error(*err),
            }
        } else {
            Tok(self.lex_keyword(word))
        }
    }

    /// Lexes either a keyword if the argument kw match a keyword or an identifier if it doesn't match
    /// a keyword. And then return it.
    pub fn lex_keyword(&self, kw: String) -> TokenType {
        match kw.as_str() {
            KW_FN => Fn,
            KW_EXTERN => Extern,
            KW_VAR => Var,
            KW_CONST => Const,
            KW_STRUCT => Struct,
            KW_ENUM => Enum,
            KW_RETURN => Return,
            KW_IF => If,
            KW_ELSE => Else,
            KW_WHILE => While,
            KW_FOR => For,
            KW_PUB => Pub,
            KW_ASYNC => Async,
            KW_AWAIT => Await,
            KW_MATCH => Match,
            KW_IMPL => Impl,
            KW_TRUE => True,
            KW_FALSE => False,
            KW_UNDEFINED => Undefined,
            KW_BREAK => Break,
            KW_CONTINUE => Continue,
            KW_PACKAGE => Package,
            KW_IMPORT => Import,
            KW_AS => As,
            _ => Ident(kw),
        }
    }

    /// Take the string containing the integer (num argument), parses it,
    /// returns the corresponding TokenType or an error if the parsing failed.
    pub fn lex_int(&mut self, num: String) -> Result<TokenType, Box<BuiltLog>> {
        match num.parse() {
            Ok(i) => Ok(Int(i)),
            Err(err) => Err(self.lctx.build_boxed(SimpleLog {
                level: LogLevel::Error,
                msg: "failed to lex integer literal".into(),
                cursor_msg: Some(err.to_string().into()),
                location: self.get_pos(),
            })),
        }
    }

    /// Lexes the input until the character that stops it (stopper argument)
    /// and returns the content
    pub fn lex_until(&mut self, stopper: char) -> String {
        let mut content = String::new();
        loop {
            match self.peek() {
                Some(c) if c == stopper => break content,
                Some(c) => {
                    content.push(c);
                    self.pop();
                }
                None => break content,
            }
        }
    }

    /// Takes a char and maps it to the corresponding escape sequence
    /// The argument 'is_string' is used to generate the error message.
    pub fn make_escape_sequence(
        &mut self,
        es: char,
        is_string: bool,
    ) -> Result<char, Box<BuiltLog>> {
        Ok(match es {
            '0' => 0x00,
            'n' => 0x0A,
            'r' => 0x0D,
            't' => 0x09,
            'x' => todo!(
                "this escape sequence will be supported but it's not actually implemented yet"
            ),
            '\\' => return Ok('\\'),
            es => {
                return Err(self.lctx.build_boxed(err::UnknownEscape {
                    escape: es,
                    is_string,
                    location: self.index - 2..self.index,
                }))
            }
        } as u8 as char)
    }

    /// Lexes the input until the end of the string literal, handles escape sequences and replace with the corresponding char.
    ///
    /// In case of an unknown character escape, both the backslash and the character following it will be ignored, and the error
    /// will be pushed to the vector, and returned with the Str tokentype contening the string but without the erronous escape
    /// character in a PartSuccess enum variant.
    pub fn lex_string_literal(&mut self) -> PartTokenResult {
        pop_expect!(self => Some('"'));
        let mut str = String::new();
        let mut errs = Vec::new();

        loop {
            match self.peek() {
                Some('"') => {
                    pop_expect!(self => Some('"'));
                    break;
                }
                Some('\\') => {
                    pop_expect!(self => Some('\\'));
                    let es = match self.pop() {
                        Some(es) => es,
                        _ => continue,
                    };
                    if es == '"' {
                        str.push(es);
                        continue;
                    }
                    match self.make_escape_sequence(es, true) {
                        Ok(res) => str.push(res),
                        Err(err) => {
                            errs.push(*err);
                            continue;
                        }
                    }
                }
                Some(c) => {
                    str.push(c);
                    pop_expect!(self => Some(c));
                }
                None => {
                    return Error(self.lctx.build(err::UnterminatedQuoteLit {
                        is_char: false,
                        location: self.prev_idx..self.index - 1,
                    }));
                }
            }
        }
        let tt = Str(str);

        if errs.is_empty() {
            Tok(tt)
        } else {
            PartSuccess(tt, errs)
        }
    }

    /// Lexe a char literal, and return it.
    pub fn lex_char_literal(&mut self) -> PartTokenResult {
        pop_expect!(self => Some('\''));
        let content: char;

        match self.peek() {
            Some('\\') => {
                pop_expect!(self => Some('\\'));

                content = match self.peek() {
                    Some(es) => 'a: {
                        pop_expect!(self => Some(es));
                        if es == '\'' {
                            break 'a es;
                        }
                        match self.make_escape_sequence(es, false) {
                            Ok(c) => c,
                            Err(err) => return Error(*err),
                        }
                    }
                    None => {
                        return Error(self.lctx.build(err::UnterminatedQuoteLit {
                            is_char: true,
                            location: self.get_pos(),
                        }));
                    }
                };
                pop_expect!(self => Some('\'');
                    return Error(self.lctx.build(err::UnterminatedQuoteLit {
                        is_char: true,
                        location: self.prev_idx..self.index - 1,
                    }))
                );
            }
            Some('\'') => {
                pop_expect!(self => Some('\''));
                if let Some('\'') = self.peek() {
                    pop_expect!(self => Some('\''));
                    return Error(self.lctx.build(SimpleLog {
                        level: LogLevel::Error,
                        msg: "char literal must be escaped `'`".into(),
                        cursor_msg: None,
                        location: self.get_pos(),
                    }));
                }
                return Error(self.lctx.build(SimpleLog {
                    level: LogLevel::Error,
                    msg: "empty char literal".into(),
                    cursor_msg: None,
                    location: self.get_pos(),
                }));
            }
            Some(c) => {
                pop_expect!(self => Some(c));

                content = c;
                pop_expect!(self => Some('\'');
                    return Error(self.lctx.build(err::UnterminatedQuoteLit {
                        is_char: true,
                        location: self.prev_idx..self.index - 1,
                    }))
                );
            }
            None => return Error(self.lctx.build(UnexpectedEOF(self.get_pos()))),
        }

        Tok(Char(content))
    }

    /// Lexes an operator if it matches an operators and return which operator was been lexed
    pub fn lex_operator(&mut self) -> Option<Operator> {
        use zom_common::operator::Operator::*;
        match (self.peek(), self.peek_nth(1)) {
            (Some(o1), wo2) => {
                let o2 = wo2.unwrap_or(' ');
                let (op, len) = match (o1, o2) {
                    ('>', '>') => (RArrow2, 2),
                    ('>', '=') => (RArrowEqual, 2),
                    ('<', '<') => (LArrow2, 2),
                    ('<', '=') => (LArrowEqual, 2),
                    ('=', '=') => (Equal2, 2),
                    ('!', '=') => (ExclamationmarkEqual, 2),
                    ('.', '*') => (DotAsterisk, 2),
                    ('|', '|') => (Pipe2, 2),
                    ('&', ..) => (Ampersand, 1),
                    ('*', ..) => (Asterisk, 1),
                    ('^', ..) => (Caret, 1),
                    ('.', ..) => (Dot, 1),
                    ('=', ..) => (Equal, 1),
                    ('!', ..) => (Exclamationmark, 1),
                    ('<', ..) => (LArrow, 1),
                    ('-', ..) => (Minus, 1),
                    ('%', ..) => (Percent, 1),
                    ('+', ..) => (Plus, 1),
                    ('>', ..) => (RArrow, 1),
                    ('/', ..) => (Slash, 1),
                    _ => return None,
                };
                self.index += len;
                Some(op)
            }
            _ => None,
        }
    }
}
