use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;

use crate::error::lexer::IllegalCharError;
use crate::error::Position;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    text: String,
    pos: usize, // position in the text
    current_char: Option<char>,
    chars: Box<Peekable<Chars<'a>>>,
    line: u32,
    column: usize,
    filename: String,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &String, filename: String) -> Lexer {
        Lexer {
            text: text.to_string(),
            pos: 0,
            current_char: None,
            chars: Box::new(text.chars().peekable()),
            line: 1,
            column: 0,
            filename,
        }
    }

    pub fn get_current_char(&self) -> char {
        if self.pos >= self.text.len() {
            return '\0';
        }
        self.current_char.unwrap()
    }

    pub fn make_tokens(&'a mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens = Vec::new();
        let mut pos = self.pos;

        'main : while let Some(ch) = self.chars.next() {
            self.pos = pos;

            match ch {
                '+' => {
                    tokens.push(Token::Operator('+'));
                    pos += 1;
                }
                '-' => {
                    tokens.push(Token::Operator('-'));
                    pos += 1;
                }
                '*' => {
                    tokens.push(Token::Operator('*'));
                    pos += 1;
                }
                '/' => {
                    if let Some('/') = self.chars.peek() {
                        self.chars.next();
                        loop {
                            let ch = self.chars.next();
                            pos += 1;
        
                            if ch == Some('\n') {
                                continue 'main;
                            }
                        }
                    }

                    tokens.push(Token::Operator('/'));
                    pos += 1;
                }
                '(' => {
                    tokens.push(Token::OpenParen);
                    pos += 1;
                }
                ')' => {
                    tokens.push(Token::CloseParen);
                    pos += 1;
                }
                ';' => {
                    tokens.push(Token::Delimiter);
                    pos += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    pos += 1;
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                }
                _ => {
                    if ch.is_whitespace() {
                        pos += 1;
                        continue;
                    }
                    return Err(Box::new(IllegalCharError::new(Position::new(
                        pos as u32,
                        self.line,
                        // TODO: Find a way to remove this magic if
                        if self.line == 1 {
                            self.column as u32
                        } else {
                            self.column as u32 - 1
                        },
                        self.filename.clone(), //TODO: Try to remove .clone()
                        self.text.clone(),
                    ))));
                }
            }
            self.column += 1;
        }

        Ok(tokens)
    }
}
