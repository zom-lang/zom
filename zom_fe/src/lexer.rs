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

    #[inline]
    pub fn match_arm(&mut self, tokens: &mut Vec<Token>, tt: TokenType) {
        tokens.push(Token::new(tt, self.pos..=self.pos));
        self.incr_pos();
    }

    pub fn make_tokens(&'a mut self) -> Result<Vec<Token>, Vec<ZomError>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errs = Vec::new();

        'main: while let Some(ch) = self.chars.next() {
            println!("ch = {ch:?}, pos = {}", self.pos);
            match ch {
                '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' => {
                    let old_pos = self.pos;
                    println!("old_pos = {old_pos}");
                    tokens.push(
                        Token::new(try_call!(self.lex_lki(ch, old_pos), errs), old_pos..=(self.pos - 1))
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
                ')' => self.match_arm(&mut tokens, CloseParen),
                '[' => self.match_arm(&mut tokens, OpenBracket),
                ']' => self.match_arm(&mut tokens, CloseBracket),
                '{' => self.match_arm(&mut tokens, OpenBrace),
                '}' => self.match_arm(&mut tokens, CloseBrace),
                ';' => self.match_arm(&mut tokens, SemiColon),
                ':' => self.match_arm(&mut tokens, Colon),
                ',' => self.match_arm(&mut tokens, Comma),
                '@' => self.match_arm(&mut tokens, At),
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

    /// This function lexes either an literal, a keyword or an identifier
    ///
    /// It takes a char in parameter because we have already "next" the iterator, so it's the actual character to put in arg.
    /// Because before it was like that :
    ///     text: `test` -> Ident("est")
    /// And after it is like that :
    ///     text: `test` -> Ident("test")
    fn lex_lki(&mut self, ch: char, old_pos: usize) -> Result<TokenType, ZomError> {
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
                match num_str.parse() {
                    Ok(i) => Ok(Int(i)),
                    Err(err) => Err(ZomError::new(
                        Position::try_from_range(
                            self.pos,
                            old_pos..=self.pos,
                            self.text.clone(),
                            self.filename.clone()
                        ),
                        err.to_string(),
                        false,
                        None,
                        vec![],
                    )),
                }
            } else {
                match num_str.parse() {
                    Ok(f) => Ok(Float(f)),
                    Err(err) => Err(ZomError::new(
                        Position::try_from_range(
                            self.pos,
                            old_pos..=self.pos - 1,
                            self.text.clone(),
                            self.filename.clone()
                        ),
                        err.to_string(), // TODO: Try to add a position to this error
                        false,
                        None,
                        vec![],
                    )),
                }
            }
        } else {
            match num_str.as_str() {
                KW_FUNC => Ok(Func),
                KW_EXTERN => Ok(Extern),
                KW_VAR => Ok(Var),
                KW_CONST => Ok(Const),
                KW_STRUCT => Ok(Struct),
                KW_ENUM => Ok(Enum),
                KW_RETURN => Ok(Return),
                KW_IF => Ok(If),
                KW_ELSE => Ok(Else),
                KW_WHILE => Ok(While),
                KW_FOR => Ok(For),
                KW_PUB => Ok(Pub),
                _ => Ok(Ident(num_str.clone())),
            }
        }
    }
}
