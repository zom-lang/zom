use crate::{token::Token, IllegalCharError};

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
        println!("CREATION OF THE LEXER");
        lex.advance();
        lex
    }

    pub fn advance(&mut self) {
        if self.pos == 0 {
            self.pos += 1;
        }
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

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, IllegalCharError> {
        let mut tokens = Vec::new();
        
        while self.current_char != None {
            println!("      IN THE LOOOOOP: {:?} !!!!!! token list {:?}", self.current_char, tokens);
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
                        println!("C'est un nombre !");
                        tokens.push(self.make_number());
                        continue;
                    }
                    let char = self.current_char.unwrap();
                    self.advance();
                    return Err(IllegalCharError::new(format!("`{char}`")));
                }
            }
        }
        /*
        loop {
            println!("IN THE LOOOOOP: {:?}", self.current_char);
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
                        println!("C'est un nombre !");
                        tokens.push(self.make_number());
                        continue;
                    }
                    let char = self.current_char.unwrap();
                    self.advance();
                    return Err(IllegalCharError::new(format!("`{char}`")));
                }
            }
            if self.current_char != None {
                break;
            }
        }*/

        Ok(tokens)
    }

    fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let mut cur_char = self.current_char.unwrap();
        
        while self.current_char != None && cur_char != ' ' && (cur_char.is_numeric() || cur_char == '.') {
            self.advance();
            println!("make number LOOP: {}", cur_char);
            println!("while condition : {}", self.current_char != None && cur_char != ' ' && (cur_char.is_numeric() || cur_char == '.'));
            if cur_char == '.' {
                if dot_count == 1 {
                    break;
                }

                dot_count += 1;
                num_str.push('.');
            }else {
                println!("make number befor push str: {}", cur_char);
                num_str.push(cur_char);
                cur_char = self.current_char.unwrap();
            }
            println!("before self.advance();");
            
        }
        /*
        loop {
            println!("make number LOOP: {}", cur_char);
            if cur_char == '.' {
                if dot_count == 1 {
                    break;
                }

                dot_count += 1;
                num_str.push('.');
            }else {
                if !(self.current_char != None && cur_char != ' ' && (cur_char.is_numeric() || cur_char == '.')) {
                    break;
                }
                println!("make number befor push str: {}", cur_char);
                num_str.push(cur_char);
                cur_char = self.current_char.unwrap();
            }
            println!("before self.advance();");
            self.advance();
        }*/

        if dot_count == 0 {
            println!("num_str = {num_str}");
            return Token::Int(num_str.parse().unwrap());
        }
        println!("num_str = {num_str}");
        return Token::Float(num_str.parse().unwrap());
    }
}
