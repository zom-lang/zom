use std::error::Error;

use zom::{run_with_args, ExitStatus};

fn main() -> Result<(), Box<dyn Error>> {
    

    let status = match run_with_args(std::env::args_os()) {
        Ok(v) => v,
        Err(err) => {
            print!("{}", err);
            std::process::exit(1)
        }
    };
    match status {
        ExitStatus::Success => {}
        ExitStatus::Error => std::process::exit(1),
    };
    Ok(())
}
