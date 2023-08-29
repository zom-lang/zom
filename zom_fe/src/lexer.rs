use std::path::Path;

use zom_common::error::ZomError;
use zom_common::token::{Token, TokenType};

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

        loop {
            match self.make_token() {
                Ok(tt) => {
                    dbg!(tt);
                }
                Err(err) => errors.push(err),
            }
        }

        todo!()
    }

    fn make_token(&mut self) -> Result<TokenType, ZomError> {
        let t = match self.peek() {
            Some('(') => TokenType::OpenParen,
            Some(')') => TokenType::CloseParen,
            Some('[') => TokenType::OpenBracket,
            Some(']') => TokenType::CloseBracket,
            Some('{') => TokenType::OpenBrace,
            Some('}') => TokenType::CloseBrace,
            Some(';') => TokenType::SemiColon,
            Some(':') => TokenType::Colon,
            Some(',') => TokenType::Comma,
            Some('@') => TokenType::At,
            _ => todo!("Add error here, illegal char"),
        };
        self.index += 1;

        Ok(t)
    }
}
