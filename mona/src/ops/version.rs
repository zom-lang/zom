use crate::ExitStatus;


pub fn version() -> Result<ExitStatus, anyhow::Error> {
    println!(
        "Mona v{}",
        env!("CARGO_PKG_VERSION")
    );

    Ok(ExitStatus::Success)
}
