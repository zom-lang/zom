//! This is the lexer of Zom
//!
//! It is entirely made for Zom, without using dependencies.

use std::iter::Peekable;
use std::str::Chars;

use zom_common::error::Position;
use zom_common::error::ZomError;
use zom_common::token::Token;

use zom_common::token::is_start_operator;

use zom_common::token::*;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    text: String,
    pos: usize, // position in the text
    chars: Box<Peekable<Chars<'a>>>,
    line: usize, // Will probably replaced with #4
    column: usize,
    filename: String,
}

#[macro_export]
macro_rules! try_call {
    ($e:expr, $err:expr) => (
        match $e {
            Ok(v) => v,
            Err(err) => {
                $err.push(err);
                continue;
            },
        }
    );
}

macro_rules! match_arm {
    ($self:expr, $tokens:expr, $tt:expr) => ({
        $tokens.push(Token::new($tt, $self.pos..=$self.pos));
        $self.incr_pos();
    });
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

    pub fn illegal_char(lexer: Lexer, ch: char) -> ZomError {
        ZomError::new(
            Some(Position::new(
                lexer.pos,
                lexer.line,
                lexer.column + 1, // + 1 because when the function is called, the column hasn't been advance
                lexer.line,
                lexer.column + 2, // + 2 because like col_start and either it will panic (see ZomError::new())
                lexer.filename,
                lexer.text,
            )),
            format!("illegal char `{}`", ch),
            false,
            Some("You should avoid using this character".to_owned()),
            vec![],
        )
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

    pub fn make_tokens(&'a mut self) -> Result<Vec<Token>, Vec<ZomError>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errs = Vec::new();

        'main: while let Some(ch) = self.chars.next() {
            match ch {
                '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' => {
                    let old_pos = self.pos;
                    let start = (self.line, self.column);
                    tokens.push(
                        Token::new(try_call!(self.lex_number(ch, start), errs), old_pos..=(self.pos - 1))
                    );
                }
                ch if is_start_operator(ch) => {
                    let window = &self.text.get(self.pos..self.pos + OP_MAX_LENGHT);

                    if window.is_none() {
                        continue;
                    }

                    let window = window.unwrap();
                    let (is_op, len) = is_operator(window);

                    if is_op {
                        tokens.push(Token::new(
                            Operator(window[..len].to_owned()),
                            self.pos..=(self.pos + len - 1),
                        ));
                        self.pos += len;
                        self.column += len;
                        continue;
                    }
                }
                '#' => loop {
                    let ch = self.chars.next();
                    self.incr_pos();

                    if ch == Some('\n') {
                        continue 'main;
                    }
                },
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
                    tokens.push(Token::new(OpenParen, self.pos..=self.pos));
                    self.incr_pos();
                }
                ')' => match_arm!(self, tokens, CloseParen),
                '[' => match_arm!(self, tokens, OpenBracket),
                ']' => match_arm!(self, tokens, CloseBracket),
                '{' => match_arm!(self, tokens, OpenBrace),
                '}' => match_arm!(self, tokens, CloseBrace),
                ';' => match_arm!(self, tokens, SemiColon),
                ':' => match_arm!(self, tokens, Colon),
                ',' => match_arm!(self, tokens, Comma),
                '@' => match_arm!(self, tokens, At),
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.pos += 1;
                }
                w if w.is_whitespace() && w != '\n' => {
                    self.incr_pos();
                    continue;
                }
                ch => //return Err(Self::illegal_char(self.clone(), ch)),
                {
                    errs.push(Self::illegal_char(self.clone(), ch));
                    self.incr_pos();
                }
            }
        }
        tokens.push(Token { tt: EOF, span: self.pos..=self.pos });

        if !errs.is_empty() {
            println!("toks = {:#?}", tokens);
            return Err(errs)
        }

        Ok(tokens)
    }

    /// This function lexes either a number literal
    fn lex_number(&mut self, ch: char, start: (usize, usize)) -> Result<TokenType, ZomError> {
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
            return if dot_count == 0 {
                match num_str.parse() {
                    Ok(i) => Ok(Int(i)),
                    Err(err) => Err(ZomError::new(
                        Some(Position::new(
                            self.pos,
                            start.0,
                            start.1 + 1,
                            self.line,
                            self.column + 1,
                            self.filename.clone(),
                            self.text.clone()
                        )),
                        "failed to lex integer literal".to_owned(),
                        false,
                        None,
                        vec![err.to_string()],
                    )),
                }
            } else {
                match num_str.parse() {
                    Ok(f) => Ok(Float(f)),
                    Err(err) => Err(ZomError::new(
                        Some(Position::new(
                            self.pos,
                            start.0,
                            start.1,
                            self.line,
                            self.column,
                            self.filename.clone(),
                            self.text.clone()
                        )),
                        "failed to lex float literal".to_owned(),
                        false,
                        None,
                        vec![err.to_string()],
                    )),
                }
            }
        }
        Ok(Lexer::lex_keyword(num_str))
    }

    /// if kw matches a keyword, the corresponding keyword is returned
    /// but if it doesn't match an ident with is returned with kw as name.
    fn lex_keyword(kw: String) -> TokenType {
        match kw.as_str() {
            KW_FUNC => Func,
            KW_EXTERN => Extern,
            KW_VAR => Var,
            KW_CONST => Const,
            KW_STRUCT => Struct,
            KW_ENUM => Enum,
            KW_RETURN => Return,
            KW_IF => If,
            KW_ELSE => Else,
            KW_WHILE => While,
            KW_FOR => For,
            KW_PUB => Pub,
            _ => Ident(kw.clone()),
        }
    }
}
