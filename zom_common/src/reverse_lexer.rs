use crate::{
    error::Position,
    token::{Token, *},
};

/// This function reversly found the position in the sourcefile,
/// using the position of the token position, the vector of tokens,
/// the sourcefile, (the filename is just to return a valid Position).
///
/// The vector of tokens needs to be the equivalent of the source file but lexed.
/// If it's not, you will have a position that point to garbage.
pub fn reverse_lexe(
    tok_pos: usize,
    tokens: Vec<Token>,
    source_file: String,
    filename: String,
) -> Result<Position, String> {
    let token = tokens.get(tok_pos);
    if token.is_none() {
        return Err("Unknow token, probably out of bounds.".to_owned());
    }

    let mut char_pos: usize = 0;
    let mut token_pos: isize = -1;
    let mut line: usize = 1;
    let mut column: usize = 0;
    let mut still_kwi = false;
    let mut still_op = false;

    let iter = source_file.chars().peekable();

    for ch in iter {
        char_pos += 1;
        column += 1;
        if still_op {
            still_op = false;
            continue;
        }

        match ch {
            '(' | ')' | '[' | ']' | '{' | '}' | ';' | ':' | ',' => {
                still_kwi = false;
                token_pos += 1;
            }
            w if w.is_ascii_whitespace() => {
                still_kwi = false;
                if w == '\n' {
                    line += 1;
                    column = 0;
                }
            }
            '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' => {
                if !still_kwi {
                    token_pos += 1;
                    still_kwi = true;
                }
            }
            ch if is_start_operator(ch) => {
                still_kwi = false;
                let window = source_file.get(char_pos - 1..char_pos - 1 + OP_MAX_LENGHT);
                if window.is_none() {
                    continue;
                }
                let window = window.unwrap();
                let (is_op, len) = is_operator(window);

                if is_op {
                    if len > 1 {
                        still_op = true;
                    }
                    token_pos += 1;
                }
            }
            _ => {
                still_kwi = false;
            }
        }

        if tok_pos == token_pos as usize {
            break;
        }
    }

    Ok(Position::new(
        char_pos,
        line,
        column - 1,
        filename,
        source_file,
    ))
}
