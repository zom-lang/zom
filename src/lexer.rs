use core::num;

use crate::token::Token;

pub const DIGITS: &str = "0123456789";

#[derive(Debug)]
pub struct Lexer<'a> {
    text: &'a str,
    pos: isize, // position in the text
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        let mut lex = Lexer {
            text,
            pos: -1,
            current_char: None,
        };
        lex.advance();
        lex
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.text.len().try_into().unwrap() {
            self.current_char = self.text.chars().nth(self.pos.try_into().unwrap());
        } else {
            self.current_char = None;
        };
    }

    pub fn get_current_char(&self) -> char {
        if self.pos >= self.text.len().try_into().unwrap() {
            return '\0';
        }
        self.current_char.unwrap()
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current_char != None {
            match &self.current_char.unwrap() {
                ' ' => self.advance(),
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Mul);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Mul);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }
                _ => {
                    if self.current_char.unwrap().is_numeric() {
                        tokens.push(self.make_number())
                    }
                }
            }
        }

        tokens
    }

    fn make_number(&self) -> Token {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let mut cur_char = self.current_char.unwrap();

        while self.current_char != None && cur_char.is_numeric() || cur_char == '.' {
            if cur_char == '.' {
                if dot_count == 1 {
                    break;
                }

                dot_count += 1;
                num_str.push('.');
                continue;
            }
            num_str.push(cur_char);
            cur_char = self.current_char.unwrap();
        }

        if dot_count == 0 {
            return Token::Int(num_str.parse().unwrap());
        }
        return Token::Float(num_str.parse().unwrap());
    }
}
