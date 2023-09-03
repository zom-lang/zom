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
    Tok(TokenType),
    Error(ZomError),
    Comment,
    Whitespace,
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
        let mut errors = Vec::new();
        let mut tokens = Vec::new();

        loop {
            let start = self.index;
            match self.make_token() {
                Tok(tt) => {
                    if tt == EOF {
                        let text_len = self.file_text().len();
                        // the length of the buffer is used for the span of the EOF, because
                        // the EOF is the last char and its 'text_len..text_len' because if for
                        // some reason we want to show the EOF in an error we can.
                        tokens.push(Token {
                            tt,
                            span: text_len..text_len,
                        });
                        break;
                    }
                    dbg!(&tt);
                    let end = self.index;
                    tokens.push(Token {
                        tt,
                        span: start..end,
                    });
                }
                Error(mut err) => {
                    let pos = Position::try_from_range(
                        self.index,
                        start..self.index,
                        self.file_text().to_string(),
                        self.file_path().to_path_buf(),
                    )
                    .expect("Unable to generate the position from the range.");
                    err.add_position(pos);
                    println!("{}", err);
                    errors.push(err)
                }
                _ => {}
            }
        }

        println!();
        for t in &tokens {
            print!("{:?}", t);
            println!(" -> {:?}", &self.file_text()[t.span.clone()]);
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tokens)
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
                        let res = self.lex_until('\n');
                        dbg!(res);
                        return Comment;
                    }
                    _ => return Tok(Operator(Operator::Div)),
                }
            }
            Some('"') => {
                self.pop();
                let mut str = String::new();

                loop {
                    match self.peek() {
                        Some(c) if c == '"' => break,
                        Some('\\') => todo!("Handle escape sequences"),
                        Some(c) => {
                            str.push(c);
                            self.pop();
                        }
                        None => break,
                    }
                }

                dbg!(&str);
                Str(str)
            }
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
}
