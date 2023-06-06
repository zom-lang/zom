use crate::ExitStatus;

use mona_compiler::target::get_target_triple;

pub fn gettarget() -> Result<ExitStatus, anyhow::Error> {
    // Yes, it's not a very good implementation but ...
    let mut res = get_target_triple()[14..].to_owned();
    res.pop();
    res.pop();

    println!("Target: {}", res);

    Ok(ExitStatus::Success)
}
