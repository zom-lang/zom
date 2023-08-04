use std::error::Error;

use crate::ExitStatus;

use zom_compiler::target::get_target_triple;

pub fn gettarget<'a>() -> Result<ExitStatus, Box<dyn Error>> {
    println!("Target: {}", get_target_triple());

    Ok(ExitStatus::Success)
}
