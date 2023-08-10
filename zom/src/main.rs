use std::backtrace::BacktraceStatus::*;
use std::panic;
use std::thread;
use std::{backtrace::Backtrace, error::Error};

use zom::{run_with_args, ExitStatus};
use zom_common::error::ZomError;

fn main() -> Result<(), Box<dyn Error>> {
    panic::set_hook(Box::new(|panic_info| {
        let thread = thread::current();
        let backtrace = Backtrace::capture();

        if let Some(name) = thread.name() {
            println!("thread '{}' {}", name, panic_info)
        } else {
            println!("{}", panic_info);
        }

        match backtrace.status() {
            Captured => {
                println!("{}", backtrace);
            }
            Disabled => println!(
                "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
            ),
            Unsupported => println!("note: backtrace is not supported."),
            _ => {}
        }

        let ice = ZomError::ice_error(panic_info.to_string());
        println!("\n{}", ice);
    }));

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
