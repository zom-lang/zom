use std::error::Error;
use std::iter::Enumerate;
use std::str::Chars;

use crate::error::lexer::IllegalCharError;
use crate::error::Position;
use crate::error::*;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    text: String,
    pos: usize, // position in the text
    current_char: Option<char>,
    iter: Option<Enumerate<Chars<'a>>>,
    line: u32,
    filename: String,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &String, filename: String) -> Lexer {
        Lexer {
            text: text.to_string(),
            pos: 0,
            current_char: None,
            iter: None,
            line: 1,
            filename,
        }
    }

    pub fn get_current_char(&self) -> char {
        if self.pos >= self.text.len() {
            return '\0';
        }
        self.current_char.unwrap()
    }

    pub fn tokenize(&'a mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        todo!("will be the tokenize method in the lexer.");
    }
}
