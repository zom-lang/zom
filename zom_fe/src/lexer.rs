use std::path::Path;

use zom_common::error::ZomError;
use zom_common::token::Token;

#[derive(Debug)]
pub struct ZomFile<'a> {
    path: &'a Path,
    text: &'a str,
}

impl<'a> ZomFile<'a> {
    pub fn new(text: &'a str, path: &'a Path) -> Self {
        Self {
            path,
            text
        }
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn path(&self) -> &Path {
        self.path
    }
}

pub struct Lexer<'a> {
    file: ZomFile<'a>,
    stack: Vec<char>
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str, path: &'a Path) -> Lexer<'a> {
        let mut stack = text.clone().chars().collect::<Vec<_>>();
        stack.reverse();

        Lexer {
            file: ZomFile::new(text, path),
            stack
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

    pub fn pop(&self) -> char {
        todo!()
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Vec<ZomError>> {
        todo!()
    }
}
