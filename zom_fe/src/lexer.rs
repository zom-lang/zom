use std::path::Path;

use zom_common::error::{Position, ZomError};
use zom_common::token::{Operator, Token, TokenType, TokenType::*};

use zom_common::token::*;

#[derive(Debug)]
pub struct ZomFile<'a> {
    path: &'a Path,
    text: &'a str,
}

impl<'a> ZomFile<'a> {
    pub fn new(text: &'a str, path: &'a Path) -> Self {
        Self { path, text }
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn path(&self) -> &Path {
        self.path
    }

    pub fn get(&self, index: usize) -> Option<char> {
        self.text.chars().nth(index)
    }
}

use PartTokenResult::*;

#[derive(Debug)]
pub enum PartTokenResult {
    /// used when the lexing is entirely successful
    Tok(TokenType),

    /// used when the lexing failed and it cannot lex anymore the content
    ///
    /// In this case, we assume the error doesn't have a position
    Error(ZomError),

    /// used when the lexing failed but it can ignore the error and keep lexing
    /// and make result to a TokenType but with multiple errors.
    ///
    /// In this case, we assume every error in the vector to have a Position
    ///
    /// (This is prefered to be used when it's possible so the development experience
    /// is not altered)
    PartSuccess(TokenType, Vec<ZomError>),

    /// used when the lexing result to a comment.
    Comment,

    /// used when the lexing results to a whitespace.
    Whitespace,
}

const POSITION_GEN_ERROR: &str = "Unable to generate the position from the range";

/// This macro pop a character using the function 'pop()'
/// and assert when the compiler is compiled is debug mode, that
/// the thing poped is what is expected.
macro_rules! pop_expect {
    ($self:expr => $expected:expr) => {{
        let poped = $self.pop();
        debug_assert_eq!(poped, $expected)
    }};
}

pub struct Lexer<'a> {
    file: ZomFile<'a>,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, path: &'a Path) -> Lexer<'a> {
        Lexer {
            file: ZomFile::new(text, path),
            index: 0,
        }
    }

    pub fn file(&self) -> &ZomFile {
        &self.file
    }

    pub fn file_path(&self) -> &Path {
        self.file.path()
    }

    pub fn file_text(&self) -> &str {
        self.file.text()
    }

    pub fn pop(&mut self) -> Option<char> {
        let c = self.peek();
        self.index += 1;
        c
    }

    pub fn peek(&self) -> Option<char> {
        self.file.get(self.index)
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Vec<ZomError>> {
        // TODO: rename this function 'lex' to avoid confusion with function 'make_token'
        let mut errors = Vec::new();
        let mut tokens = Vec::new();

        loop {
            let start = self.index;
            match self.make_token() {
                Tok(tt) => {
                    if self.push_token(&mut tokens, tt, start) {
                        break;
                    }
                }
                Error(mut err) => {
                    let pos = Position::try_from_range(
                        self.index,
                        start..self.index,
                        self.file_text().to_string(),
                        self.file_path().to_path_buf(),
                    )
                    .expect(POSITION_GEN_ERROR);
                    err.add_position(pos);

                    errors.push(err);
                }
                PartSuccess(tt, mut errs) => {
                    dbg!(&tt);
                    debug_assert!(!errs.is_empty(), "the list of errors shouldn't be empty");

                    #[cfg(debug_assertions)]
                    {
                        for error in &errs {
                            assert!(error.has_pos(), "error should have position");
                        }
                    }

                    errors.append(&mut errs);
                    if self.push_token(&mut tokens, tt, start) {
                        break;
                    }
                }
                Comment | Whitespace => {}
            }
        }

        println!("\n~~~  SEPARTOR  ~~~");
        for t in &tokens {
            print!("{:?}", t);
            println!(" -> {:?}", &self.file_text()[t.span.clone()]);
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tokens)
    }

    /// This functions takes a vector of tokens, and push a Token containing the given TokenType
    /// with the start arg and the index at that moment.
    ///
    /// And if the TokenType is a EOF it returns true, either false.
    fn push_token(&self, tokens: &mut Vec<Token>, tt: TokenType, start: usize) -> bool {
        if tt == EOF {
            let text_len = self.file_text().len();
            // the length of the buffer is used for the span of the EOF, because
            // the EOF is the last char and its 'text_len..text_len' because if for
            // some reason we want to show the EOF in an error we can.
            tokens.push(Token {
                tt,
                span: text_len..text_len,
            });
            return true;
        }
        dbg!(&tt);
        let end = self.index;
        tokens.push(Token {
            tt,
            span: start..end,
        });

        false
    }

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
                    _ => return Tok(Operator(Operator::Div)),
                }
            }
            Some('"') => return self.lex_string_literal(),
            Some('A'..='Z' | 'a'..='z' | '_' | '0'..='9') => return self.lex_word(),
            Some(w) if w.is_whitespace() => {
                self.index += 1;
                return Whitespace;
            }
            Some(c) => {
                self.index += 1;
                return Error(ZomError::new(
                    None,
                    format!("unknown start of token, '{}'", c),
                    false,
                    None,
                    vec![],
                ));
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
            self.pop();
        }
        dbg!((&word, is_numeric));
        (word, is_numeric)
    }

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

    pub fn lex_keyword(&self, kw: String) -> TokenType {
        match kw.as_str() {
            KW_FUNC => Func,
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
            _ => Ident(kw),
        }
    }

    /// Take the string containing the integer (num argument), parses it,
    /// returns the corresponding TokenType or an error if the parsing failed.
    pub fn lex_int(&self, num: String) -> Result<TokenType, Box<ZomError>> {
        match num.parse() {
            Ok(i) => Ok(Int(i)),
            Err(err) => Err(Box::new(ZomError::new(
                None,
                "failed to lex integer literal".to_owned(),
                false,
                None,
                vec![err.to_string()],
            ))),
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
    pub fn make_escape_sequence(&self, es: char) -> Result<char, Box<ZomError>> {
        Ok(match es {
            '0' => 0x00,
            'n' => 0x0A,
            'r' => 0x0D,
            't' => 0x09,
            'x' => todo!(
                "this escape sequence will be supported but it's not actually implemented yet"
            ),
            '\\' => return Ok('\\'),
            es => return Err(Box::new(ZomError::new(
                    Some(Position::try_from_range(
                        self.index,
                        self.index - 1..self.index,
                        self.file_text().to_string(),
                        self.file_path().to_path_buf()
                    ).expect(POSITION_GEN_ERROR)),
                    format!("unknown character escape sequence: '{}'", es),
                    false,
                    Some(r#"supported escapse sequence are, '\0', '\n', '\r', '\t'; and '\'' or '\"', depending if it is a string or char literal."#.to_string()),
                    vec![]
                )))
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
        let mut errors = Vec::new();

        loop {
            match self.peek() {
                Some(c) if c == '"' => {
                    pop_expect!(self => Some('"'));
                    break;
                }
                Some('\\') => {
                    pop_expect!(self => Some('\\'));
                    let es = match self.pop() {
                        Some(es) => es,
                        _ => todo!("Unterminated string literal"),
                    };
                    if es == '"' {
                        str.push(es);
                        continue;
                    }
                    dbg!(es);
                    match self.make_escape_sequence(es) {
                        Ok(res) => str.push(res),
                        Err(err) => errors.push(*err),
                    }
                }
                Some(c) => {
                    str.push(c);
                    pop_expect!(self => Some(c));
                }
                None => todo!("Unterminated string literal"),
            }
        }
        dbg!(&str);
        let tt = Str(str);
        if !errors.is_empty() {
            return PartSuccess(tt, errors);
        }

        Tok(tt)
    }
}
