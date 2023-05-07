use crate::token::Token;

use super::token;

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

    // pub fn make_tokens(&self)<V> -> Vec<Token<V>> {
    //     let tokens = Vec::new();
    //
    //     while self.current_char != None {
    //         match self.current_char {
    //             ' ' => self.advance(),
    //             '+' => {
    //                 tokens.push(Token::new(token::TT_PLUS, None))
    //             }
    //             _ => {}
    //         }
    //     }
    //
    //     tokens
    // }
}
