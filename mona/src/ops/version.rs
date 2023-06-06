use crate::ExitStatus;

pub fn version() -> Result<ExitStatus, anyhow::Error> {
    print!(
        "Mona v{} ",
        env!("CARGO_PKG_VERSION")
    );

    cfg!(debug_assertions).then(|| {
        print!("in DEBUG binary")
    });

    println!();

    Ok(ExitStatus::Success)
}
