use mona::run;
use std::io::{self, Write};

fn main() {
    println!("Mona {}, to exit enter `.quit`", env!("CARGO_PKG_VERSION"));

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
        if input_buf.as_str() == ".quit" {
            break;
        }

        let result = run("<stdin>".to_string(), input_buf);

        match result {
            Ok(res) => {
                println!("{:?}", res);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }

        input_buf = String::new();
    }
    println!("\n See you soon! ;)");
}
