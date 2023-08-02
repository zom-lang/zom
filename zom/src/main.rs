use zom::{run_with_args, ExitStatus};

fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::try_init()
        .expect("Error occurs at the initialization of pretty_env_logger.");
    println!("GIT_HASH={}", env!("GIT_HASH"));
    println!("TARGET_TRIPLE={}", env!("TARGET_TRIPLE"));
    println!("COMPILED_DATE={}", env!("COMPILED_DATE"));
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
