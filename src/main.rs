use mona::run;
use std::io::{self, Write};

fn main() {
    println!("Mona {}, to exit enter `exit`", env!("CARGO_PKG_VERSION"));

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

        //TODO: Add the possibility to safely quit mona with Ctrl + C.
        if input_buf.starts_with("exit") {
            break;
        }

        let result = run(input_buf);

        match result {
            Ok(res) => {
                println!("{:?}", res);
            }
            Err(err) => {
                match err.source() {
                    Some(source) => {
                        eprintln!("ERR: {source}: {err}");
                    }
                    None => {
                        eprintln!("ERR: {err}");
                    }
                }
            }
        }

        input_buf = String::new();
    }
    println!("\n See you soon! ;)");
}
