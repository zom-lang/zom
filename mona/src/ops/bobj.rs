use std::path::PathBuf;

use crate::ExitStatus;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Path to the Mona source file
    source_file: PathBuf,

    /// Path to where the object file will go
    #[clap(short, long)]
    output_file: Option<PathBuf>,

    /// Path to where the object file will go
    #[clap(short = 'O', long, default_value_t = 2)]
    // TODO: Change this to the actual things later.
    optimization_level: usize,

    /// Emits IR instead of a *.o
    #[clap(long, short, action = clap::ArgAction::SetTrue)] 
    emit_ir: bool,
}

pub fn build(args: Args) -> Result<ExitStatus, anyhow::Error> {
    println!("{:?}", args);
    todo!()
}
