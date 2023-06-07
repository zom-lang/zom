//! This is the lexer of Mona
//!
//! It is entirely made for Mona, without using dependencies.

use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;

use std::mem;

use crate::token::Token;
use mona_common::error::lexer::IllegalCharError;
use mona_common::error::Position;

use super::token::is_start_operator;

use super::token::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    text: String,
    pos: usize, // position in the text
    chars: Box<Peekable<Chars<'a>>>,
    line: u32,
    column: usize,
    filename: String,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &str, filename: String) -> Lexer {
        Lexer {
            text: text.to_string(),
            pos: 0,
            chars: Box::new(text.chars().peekable()),
            line: 1,
            column: 0,
            filename,
        }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn make_tokens(&'a mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens = Vec::new();

        'main: while let Some(ch) = self.chars.next() {
            match ch {
                '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' => {
                    tokens.push(self.lex_lki(ch)?);
                }
                ch if is_start_operator(ch) => {
                    let window = &self.text.get(self.pos..self.pos + OP_MAX_LENGHT);

                    if let None = window {
                        continue;
                    }

                    let window = window.unwrap().trim();
                    let (is_op, len) = is_operator(window);

                    if is_op {
                        tokens.push(Operator(window[..len].to_owned()));
                        self.pos += len;
                        continue;
                    }
                }
                '#' => {
                    self.chars.next();
                    loop {
                        let ch = self.chars.next();
                        self.pos += 1;

                        if ch == Some('\n') {
                            continue 'main;
                        }
                    }
                }
                '(' => {
                    tokens.push(Token::OpenParen);
                    self.pos += 1;
                }
                ')' => {
                    tokens.push(Token::CloseParen);
                    self.pos += 1;
                }
                '[' => {
                    tokens.push(Token::OpenBracket);
                    self.pos += 1;
                }
                ']' => {
                    tokens.push(Token::CloseBracket);
                    self.pos += 1;
                }
                '{' => {
                    tokens.push(Token::OpenBrace);
                    self.pos += 1;
                }
                '}' => {
                    tokens.push(Token::CloseBrace);
                    self.pos += 1;
                }
                ';' => {
                    tokens.push(Token::Delimiter);
                    self.pos += 1;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.pos += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.pos += 1;
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.pos += 1;
                }
                _ => {
                    self.pos += 1;
                    if ch.is_whitespace() {
                        continue;
                    }
                    return Err(Box::new(IllegalCharError::new(Position::new(
                        self.pos as u32,
                        self.line,
                        self.column as u32,
                        mem::take(&mut self.filename),
                        mem::take(&mut self.text),
                    ))));
                }
            }
            self.column += 1;
        }

        Ok(tokens)
    }

    /// This function lexes either an literal, a keyword or an identifier
    ///
    /// It takes a char in parameter because we have already "next" the iterator, so it's the actual character to put in arg.
    /// Because before it was like that :
    ///     text: `test` -> Ident("est")
    /// And after it is like that :
    ///     text: `test` -> Ident("test")
    fn lex_lki(&mut self, ch: char) -> Result<Token, Box<dyn Error>> {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let mut is_numeric = true;
        let mut ch = ch;

        loop {
            self.pos += 1;
            if ch == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    break;
                }
            } else if ch.is_whitespace() || !ch.is_alphanumeric() && ch != '_' {
                is_numeric = false;
                break;
            } else if !ch.is_numeric() {
                is_numeric = false;
            }
            if self.pos > self.text.len() {
                break
            }else {
                num_str.push(ch);
            }
            if let Some(ch_peek) = self.chars.peek() {
                if ch_peek.is_whitespace() || !ch_peek.is_alphanumeric() && ch_peek != &'_' {
                    break;
                }else if let Some(char) = self.chars.next() {
                    ch = char;
                } else {
                    break;
                }
            }
        }

        if is_numeric {
            if dot_count == 0 {
                Ok(Token::Int(num_str.parse()?))
            } else {
                Ok(Token::Float(num_str.parse()?))
            }
        } else {
            match num_str.as_str() {
                KEY_FUNC => Ok(Token::Func),
                KEY_EXTERN => Ok(Token::Extern),
                KEY_LET => Ok(Token::Let),
                _ => Ok(Token::Ident(num_str.clone())),
            }
        }
    }
}
