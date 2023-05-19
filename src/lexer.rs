use std::error::Error;
use std::iter::Enumerate;
use std::str::Chars;

use crate::error::lexer::IllegalCharError;
use crate::error::Position;
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

    pub fn make_tokens(&'a mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens = Vec::new();

        self.iter = Some(self.text.chars().enumerate());

        while let Some((mut _idx, mut _ch)) = self.iter.as_mut().unwrap().next() {
            self.pos = _idx;
            match _ch {
                '0'..='9' | '.' | 'A'..='z' => {
                    let num = Self::make_word(
                        &self.text,
                        self.pos,
                    )?;

                    let (tok, new_pos) = num;

                    for _ in 0..(new_pos.0 - 1) {
                        (_idx, _ch) = self
                            .iter
                            .as_mut()
                            .unwrap()
                            .next()
                            .expect("ERR: running out of bounds")
                    }
                    tokens.push(tok);
                }
                '+' => tokens.push(Token::Operator("+".to_string())),
                '-' => tokens.push(Token::Operator("-".to_string())),
                '*' => tokens.push(Token::Operator("*".to_string())),
                '/' => tokens.push(Token::Operator("/".to_string())),
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                ';' => tokens.push(Token::Delimiter),
                ',' => tokens.push(Token::Comma),
                _ => {
                    if _ch.is_whitespace() {
                        continue;
                    }
                    return Err(Box::new(IllegalCharError::new(Position::new(
                        _idx as u32,
                        self.line,
                        _idx as u32,
                        self.filename.clone(), //TODO: Try to remove .clone()
                        self.text.clone(),
                    ))));
                }
            }
        }

        Ok(tokens)
    }

    fn set_current_char(text: &str, pos: usize) -> Option<char> {
        // TODO: rewrite this function I think it's not very efficient ...
        for (i, c) in text.chars().enumerate() {
            if i == pos {
                return Some(c);
            }
        }
        None
    }

    /// The name of the function isn't very appropriate but this is what this is.
    /// This function take the text (code) and a position.
    /// 
    /// This return a Result -> 
    ///     The token, can be if it's only a int or float, a Token::Float or Int with the correct inner value,
    ///                or if it's not numeric, that returns either Token::Func or Token::Extern if that match with the keyword,
    ///                or if nothing is "true" that returns a Token::Ident
    pub fn make_word(
        text: &str,
        pos: usize,
    ) -> Result<(Token, (usize, char)), Box<dyn Error>> {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let mut pos: usize = pos;
        let mut curr_char: Option<char> = Self::set_current_char(text, pos);
        let mut is_numeric = true;

        while let Some(ch) = curr_char {
            if ch == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    break;
                }
            } else if ch.is_whitespace() {
                break;
            } else if !ch.is_numeric() {
                is_numeric = false;
            } 
            num_str.push(ch);
            pos += 1;
            curr_char = Self::set_current_char(text, pos);
        }

        curr_char = Self::set_current_char(text, pos - 1);

        let val = if is_numeric {
            if dot_count == 0 {
                Ok((Token::Int(num_str.parse()?), (num_str.len(), curr_char.unwrap())))
            }else {
                Ok((Token::Float(num_str.parse()?), (num_str.len(), curr_char.unwrap())))
            }
        }else {
            match num_str.as_str() {
                "func" => Ok((Token::Func, (num_str.len(), curr_char.unwrap()))),
                "extern" => Ok((Token::Extern, (num_str.len(), curr_char.unwrap()))),
                _ => Ok((Token::Ident(num_str.clone()), (num_str.len(), curr_char.unwrap())))
            }
        };

        val
    }
}
