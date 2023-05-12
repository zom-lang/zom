use mona::run;
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
            .help(r"Not quiet implement"),
    )
    .arg(
        Arg::new("lexer")
            .short('l')
            .long("lexer")
            .action(ArgAction::SetTrue)
            .help(r"Show result of the lexer"),
    )
    .arg(
        Arg::new("parser")
            .short('p')
            .long("parser")
            .action(ArgAction::SetTrue)
            .help(r"Show the result of the parser"),
    )
    .arg(
        Arg::new("interpreter")
            .short('i')
            .long("interpreter")
            .action(ArgAction::SetTrue)
            .help(r"Show the result of the interpreter")
    )
    .get_matches();

    let lexer_flag = matches.get_flag("lexer");
    let parser_flag = matches.get_flag("parser");
    let verbose_flag = matches.get_flag("verbose");
    let interpreter_flag = !matches.get_flag("interpreter");
    let file = matches.get_one::<String>("file");

    if verbose_flag {
        print!(" Flags: ");
        if lexer_flag {
            print!("Lexer,");
        }
        if parser_flag {
            print!("Parser,");
        }
        if interpreter_flag {
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
