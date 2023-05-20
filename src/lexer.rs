use std::error::Error;

// use crate::error::lexer::IllegalCharError;
// use crate::error::Position;
use crate::token::Token;

macro_rules! regex {
    ($re:expr) => {
        ::regex::Regex::new($re).unwrap()
    };
}

#[derive(Debug)]
pub struct Lexer {
    text: String,
    pos: usize, // position in the text
    current_char: Option<char>,
    _filename: String,
}

impl Lexer {
    pub fn new(text: &String, _filename: String) -> Lexer {
        Lexer {
            text: text.to_string(),
            pos: 0,
            current_char: None,
            _filename,
        }
    }

    pub fn get_current_char(&self) -> char {
        if self.pos >= self.text.len() {
            return '\0';
        }
        self.current_char.unwrap()
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        // regex for commentaries (start with #, end with the line end)
        let comment_re = regex!(r"(?m)//.*\n");
        // remove commentaries from the input stream
        let preprocessed = comment_re.replace_all(self.text.as_str(), "\n");

        let mut result = Vec::new();

        // regex for token, just union of straightforward regexes for different token types
        // operators are parsed the same way as identifier and separated later
        let token_re = regex!(concat!(
            r"(?P<ident>\p{Alphabetic}\w*)|",
            r"(?P<number>\d+\.?\d*)|",
            r"(?P<delimiter>;)|",
            r"(?P<oppar>\()|",
            r"(?P<clpar>\))|",
            r"(?P<comma>,)|",
            r"(?P<operator>\S)"));

        for cap in token_re.captures_iter(preprocessed.to_string().as_str()) {
            let token = if cap.name("ident").is_some() {
                match &cap["ident"] {
                    "func" => Token::Func,
                    "extern" => Token::Extern,
                    ident => Token::Ident(ident.to_string())
                }
            } else if cap.name("number").is_some() {
                Token::Int(cap["number"].parse().expect("Lexer wasn't able to successfully parse the number."))
            } else if cap.name("delimiter").is_some() {
                Token::Delimiter
            } else if cap.name("oppar").is_some() {
                Token::OpenParen
            } else if cap.name("clpar").is_some() {
                Token::CloseParen
            } else if cap.name("comma").is_some() {
                Token::Comma
            } else {
                Token::Operator(cap["operator"].to_string())
            };

            result.push(token)
        }

        Ok(result)
    }
}
