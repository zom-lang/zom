use std::iter::Enumerate;
use std::str::Chars;

use crate::{token::Token, IllegalCharError};

pub const DIGITS: &str = "0123456789";

#[derive(Debug)]
pub struct Lexer<'a>{
    text: String,
    pos: usize, // position in the text
    current_char: Option<char>,
    iter: Option<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &String) -> Lexer {
        Lexer {
            text: text.to_string(),
            pos: 0,
            current_char: None,
            iter: None
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.text.len().try_into().unwrap() {
            let pos: usize = self.pos.try_into().unwrap();
            println!("  1 in advance(..), before {:?} idx {}, idx usize {}", self.current_char, self.pos, pos);
            self.current_char = self.text.chars().nth(pos);
            println!("  1 in advance(..), after {:?} idx {}, idx usize {}", self.current_char, self.pos, pos);
        } else {
            println!("  2 in advance(..), before {:?} idx {}", self.current_char, self.pos);
            self.current_char = None;
            println!("  2 in advance(..), after {:?} idx {}", self.current_char, self.pos);
        };
    }

    pub fn get_current_char(&self) -> char {
        if self.pos >= self.text.len().try_into().unwrap() {
            return '\0';
        }
        self.current_char.unwrap()
    }

    pub fn make_tokens(&'a mut self) -> Result<Vec<Token>, IllegalCharError> {
        let mut tokens = Vec::new();
        
        self.iter = Some(self.text.chars().enumerate());

        for (idx, ch) in self.iter.as_mut().unwrap() {
            self.pos = idx;
            match ch {
                ' ' => {}
                '0'..='9' | '.' => {
                    tokens.push(self.make_number());
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                _ => {
                    return Err(IllegalCharError::new(format!("`{ch}`")));
                }
            }
        }

        Ok(tokens)
    }

    fn set_current_char(&mut self) {
        for (i, c) in self.text.chars().enumerate() {
            if i == self.pos {
                self.current_char = Some(c);
                return;
            }
        }
    }

    pub fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count = 0;

        self.set_current_char();
    
        while let Some(ch) = self.current_char {
            if ch == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    break;
                }
            } else if !DIGITS.contains(ch) {
                break;
            } 
            num_str.push(ch);
            self.advance();
        }
    
        if dot_count == 0 {
            Token::Int(num_str.parse().unwrap())
        } else {
            Token::Float(num_str.parse().unwrap())
        }
    }
}