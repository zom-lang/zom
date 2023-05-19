use mona::driver::{Flags, main_loop};

use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("file").help("source file to be executed"))
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
        .get_matches();

    let file = matches.get_one::<String>("file");

    let mut flags = Flags::new(
        matches.get_flag("lexer"),
        matches.get_flag("parser"),
        matches.get_flag("verbose"),
    );
    println!("Mona {}, to exit enter `.quit`", env!("CARGO_PKG_VERSION"));

    cfg!(debug_assertions).then(|| {
        println!("  You're in a debug binary, if it's not intentional, you should change.");
        flags.lexer = true;
        flags.parser = true;
        flags.verbose = true;
    });

    if flags.verbose {
        print!(" Flags: ");
        if flags.lexer {
            print!("Lexer,");
        }
        if flags.parser {
            print!("Parser,");
        }
        if file.is_some() {
            print!("File `{}`,", file.unwrap())
        }
        println!();
    }

    main_loop(flags);

    println!("\n See you soon! ;)");
}
