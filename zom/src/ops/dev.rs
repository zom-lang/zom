use std::io::{self, stdout, Write};
use anyhow::anyhow;
use zom_fe::lexer::Lexer;

use crate::ExitStatus;

pub fn dev() -> Result<ExitStatus, anyhow::Error> {
    println!("Development command.");

    let mut buffer = String::new();

    print!("input: ");
    stdout().flush().expect("ERR: Flush the output failed.");
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(err) => return Err(anyhow!(format!("{}", err)))
    }

    println!("buffer = {:?}", buffer);

    let mut lexer = Lexer::new(buffer.as_str(), "<dev_cmd>.zom".to_string());

    match lexer.make_tokens() {
        Ok(toks) => println!("toks = {:?}", toks),
        Err(err) => return Err(anyhow!(format!("{}", err)))
    }

    Ok(ExitStatus::Success)
}
