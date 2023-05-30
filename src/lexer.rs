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
    pub fn new(text: &str, filename: String) -> Lexer {
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
                '.' | '0'..='9' | 'A'..='z' => {
                    tokens.push(self.lex_lki(ch)?);
                },
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
                        self.column as u32,
                        self.filename.clone(), //TODO: Try to remove .clone()
                        self.text.clone(),
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
            println!("IN THE LKI loop : ch = {ch}");
            if ch == '.' {
                dot_count += 1;
                self.pos += 1;
                if dot_count > 1 {
                    break;
                }
            } else if ch.is_whitespace() || !ch.is_alphanumeric() && ch != '_' {
                is_numeric = false;
                break;
            } else if !ch.is_numeric() {
                is_numeric = false;
            }
            num_str.push(ch);
            self.pos += 1;
            if let Some(char) = self.chars.next() {
                ch = char;
            }else {
                break;
            }
        }

        let val = if is_numeric {
            if dot_count == 0 {
                Ok(Token::Int(num_str.parse()?))
            } else {
                Ok(Token::Float(num_str.parse()?))
            }
        } else {
            match num_str.as_str() {
                "func" => Ok(Token::Func),
                "extern" => Ok(Token::Extern),
                _ => Ok(Token::Ident(num_str.clone()))
            }
        };

        val
    }
}
