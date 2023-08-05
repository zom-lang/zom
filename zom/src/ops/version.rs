use std::error::Error;

use crate::ExitStatus;

pub fn version() -> Result<ExitStatus, Box<dyn Error>> {
    print!("zomc {}", env!("CARGO_PKG_VERSION"));

    cfg!(debug_assertions).then(|| print!("in DEBUG binary"));

    println!();

    Ok(ExitStatus::Success)
}
