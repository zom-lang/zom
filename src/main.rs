use std::io::{self, Write};

use mona::Token;

fn main() {
    println!("Mona {}", env!("CARGO_PKG_VERSION"));

    let mut input_buf = String::new();
    loop {
        print!("~> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input_buf) {
            Ok(_len) => {}
            Err(err) => {
                println!("ERR: {}", err);
                continue;
            }
        }
        input_buf = String::from(input_buf.trim());

        println!("{}", input_buf);

        let tok: Token<&str, _> = Token::new("INT", Some(10));

        println!("{}", tok);

        if input_buf.starts_with("exit") {
            break;
        }

        input_buf = String::new();
    }
    println!("\n See you soon! ;)");
}
