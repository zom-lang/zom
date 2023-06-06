use crate::ExitStatus;

use mona_compiler::target::get_target_triple;

pub fn gettarget() -> Result<ExitStatus, anyhow::Error> {
    // Yes, it's not a very good implementation but ...
    println!("Target: {}", get_target_triple());

    Ok(ExitStatus::Success)
}
