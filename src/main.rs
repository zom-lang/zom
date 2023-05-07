use std::io::{self, Write};

fn main() {
    println!("Mona {}", env!("CARGO_PKG_VERSION"));

    let mut input_buf = String::new();
    loop {
        print!("~>> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input_buf) {
            Ok(_len) => {}
            Err(err) => {
                println!("ERR: {}", err);
                continue;
            }
        }
        input_buf = String::from(input_buf.trim());

        println!("input_buf = `{}`", input_buf);

        if input_buf.starts_with("exit") {
            println!("Goodbey !");
            return;
        }
        input_buf = String::new();
    }
}
