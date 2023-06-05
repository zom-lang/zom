use std::path::PathBuf;

use crate::ExitStatus;

#[derive(clap::Args)]
pub struct Args {
    /// Path to the Mona source file
    file_path: PathBuf,

    /// Path to where the object file will go
    #[clap(short, long)]
    output_file: Option<PathBuf>,

    /// Path to where the object file will go
    #[clap(short = 'O', long, default_value_t = 2)]
    // TODO: Change this to the actual things later.
    optimization_level: usize,

    /// Emits IR instead of a *.o
    #[clap(long)] // TODO: turn it into a flag
    emit_ir: bool,
}

pub fn build(args: Args) -> Result<ExitStatus, anyhow::Error> {
    todo!()
}
