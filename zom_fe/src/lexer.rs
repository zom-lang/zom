//! This is the lexer of Zom
//!
//! It is entirely made for Zom, without using dependencies.

use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;

use std::mem;

use zom_common::error::lexer::IllegalCharError;
use zom_common::error::Position;
use zom_common::token::Token;

use zom_common::token::is_start_operator;

use zom_common::token::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    text: String,
    pos: usize, // position in the text
    chars: Box<Peekable<Chars<'a>>>,
    line: usize,
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

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }

    #[inline]
    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    #[inline]
    pub fn incr_pos(&mut self) {
        self.pos += 1;
        self.column += 1;
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

                    if window.is_none() {
                        continue;
                    }

                    let window = window.unwrap();
                    let (is_op, len) = is_operator(window);

                    if is_op {
                        tokens.push(Operator(window[..len].to_owned()));
                        self.pos += len;
                        self.column += len;
                        continue;
                    }
                }
                '#' => {
                    self.chars.next();
                    loop {
                        let ch = self.chars.next();
                        self.incr_pos();
    
                        if ch == Some('\n') {
                            continue 'main;
                        }
                    }
                }
                '(' => {
                    if let Some('*') = self.chars.peek() {
                        // Eat the `*` char
                        self.chars.next();
                        self.incr_pos();
                        let mut comment = String::new();

                        'comment: loop {
                            let ch = self.chars.next();

                            if ch == Some('\n') {
                                self.line += 1;
                                self.column = 0;
                                self.pos += 1;
                                continue 'comment;
                            } else {
                                self.incr_pos();
                            }

                            let window = &self.text.get(self.pos..self.pos + 2);

                            if ch.is_none() {
                                break 'comment;
                            }

                            if window.is_none() {
                                continue;
                            }
                            let window = window.unwrap();

                            if window == "*)" {
                                self.incr_pos();
                                break 'comment;
                            }
                            comment.push(ch.unwrap());
                        }
                        self.chars.next();
                        self.incr_pos();
                        continue 'main;
                    }
                    tokens.push(Token::OpenParen);
                    self.incr_pos();
                }
                ')' => {
                    tokens.push(Token::CloseParen);
                    self.incr_pos();
                }
                '[' => {
                    tokens.push(Token::OpenBracket);
                    self.incr_pos();
                }
                ']' => {
                    tokens.push(Token::CloseBracket);
                    self.incr_pos();
                }
                '{' => {
                    tokens.push(Token::OpenBrace);
                    self.incr_pos();
                }
                '}' => {
                    tokens.push(Token::CloseBrace);
                    self.incr_pos();
                }
                ';' => {
                    tokens.push(Token::SemiColon);
                    self.incr_pos();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.incr_pos();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.incr_pos();
                }
                '@' => {
                    tokens.push(Token::At);
                    self.incr_pos();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.pos += 1;
                }
                w if w.is_whitespace() && w != '\n' => {
                    self.incr_pos();
                    continue;
                }
                _ => {
                    return Err(Box::new(IllegalCharError::new(
                        Position::new(
                            self.pos,
                            self.line,
                            self.column,
                            mem::take(&mut self.filename),
                            mem::take(&mut self.text),
                        ),
                        ch,
                    )));
                }
            }
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
            self.incr_pos();
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
                break;
            } else {
                num_str.push(ch);
            }
            if let Some(ch_peek) = self.chars.peek() {
                if ch_peek.is_whitespace() || !ch_peek.is_alphanumeric() && ch_peek != &'_' {
                    break;
                } else if let Some(char) = self.chars.next() {
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
                KEY_VAR => Ok(Token::Var),
                KEY_CONST => Ok(Token::Const),
                KEY_STRUCT => Ok(Token::Struct),
                KEY_ENUM => Ok(Token::Enum),
                KEY_RETURN => Ok(Token::Return),
                KEY_IF => Ok(Token::If),
                KEY_ELSE => Ok(Token::Else),
                KEY_WHILE => Ok(Token::While),
                KEY_FOR => Ok(Token::For),
                KEY_PUB => Ok(Token::Pub),
                _ => Ok(Token::Ident(num_str.clone())),
            }
        }
    }
}
