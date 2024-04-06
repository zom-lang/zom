use std::{
    error::Error,
    // mem,
    path::PathBuf,
};

use crate::ExitStatus;

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    /// Path to the Zom source file
    source_file: PathBuf,

    /// Path to where the object file will go
    #[clap(short, long)]
    output_file: Option<PathBuf>,

    /// LLVM level of optimization (not implemented yet)
    #[clap(short = 'O', long, default_value_t = 2)]
    // TODO: Change this to the actual things later.
    optimization_level: usize,

    /// Emits IR instead of a *.o
    #[clap(long, short, action = clap::ArgAction::SetTrue)]
    emit_ir: bool,

    /// Print verbose ouput if enabled.
    #[clap(long, short = 'V', action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

pub fn build(_args: Args) -> Result<ExitStatus, Box<dyn Error>> {
    todo!("Will be removed later with #42")
}
