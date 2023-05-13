use mona::{run, Flags};
use std::io::{self, Write};

use clap::{command, Arg, ArgAction};

fn main() {
    println!("Mona {}, to exit enter `.quit`", env!("CARGO_PKG_VERSION"));

    let matches = command!() // requires `cargo` feature
    .arg(
        Arg::new("file")
            .help("source file to be executed")
    )
    .arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue)
            .help("Not quiet implement"),
    )
    .arg(
        Arg::new("lexer")
            .short('l')
            .long("lexer")
            .action(ArgAction::SetTrue)
            .help("Show result of the lexer"),
    )
    .arg(
        Arg::new("parser")
            .short('p')
            .long("parser")
            .action(ArgAction::SetTrue)
            .help("Show the result of the parser"),
    )
    .arg(
        Arg::new("interpreter")
            .short('i')
            .long("interpreter")
            .action(ArgAction::SetTrue)
            .help("Show the result of the interpreter. By default, set to true.")
    )
    .get_matches();

    let file = matches.get_one::<String>("file");

    let flags = Flags::new(
        matches.get_flag("lexer"), 
        matches.get_flag("parser"), 
        !matches.get_flag("interpreter"), 
        matches.get_flag("verbose")
    );

    if flags.verbose {
        print!(" Flags: ");
        if flags.lexer {
            print!("Lexer,");
        }
        if flags.parser {
            print!("Parser,");
        }
        if flags.interpreter {
            print!("Interpreter,");
        }
        if file.is_some() {
            print!("File `{}`,", file.unwrap())
        }
        println!();
    }

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
                res.print_res(flags);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }

        input_buf = String::new();
    }
    println!("\n See you soon! ;)");
}
