use crate::ExitStatus;

use mona_compiler::target::get_target_triple;

pub fn gettarget() -> Result<ExitStatus, anyhow::Error> {
    println!("Target: {}", get_target_triple());

    Ok(ExitStatus::Success)
}
