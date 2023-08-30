use std::path::Path;

use zom_common::error::{Position, ZomError};
use zom_common::token::{Token, TokenType, TokenType::*};

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
                    dbg!(&tt);
                    if tt == EOF {
                        break;
                    }
                    let end = self.index;
                    tokens.push(Token {
                        tt,
                        span: start..=end - 1, // we substact one from the span because the lexer works
                                               // with non-range inclusive and the token stores a range
                                               // inclusive.
                    })
                }
                Error(err) => {
                    // TODO: add position to errors here
                    println!("{}", err);
                    errors.push(err)
                }
                _ => {
                    println!("")
                }
            }
        }

        println!();
        for t in tokens {
            println!("{:?} -> {:?}", t.tt, &self.file_text()[t.span]);
        }

        todo!()
    }

    fn make_token(&mut self) -> PartTokenResult {
        let t = match self.peek() {
            Some('A'..='Z' | 'a'..='z' | '_') => {
                println!("An ident / keyword / number");
                todo!()
            }
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
            Some(w) if w.is_whitespace() => {
                self.index += 1;
                return Whitespace;
            }
            Some(c) => {
                self.index += 1;
                return Error(ZomError::new(
                    None,
                    format!("illegal char '{}'", c),
                    false,
                    Some("You should avoid using non-ascii characters, they are only supported in string literrals".to_string()),
                    vec![]
                ));
            }
            None => EOF,
        };
        self.index += 1;

        Tok(t)
    }
}
