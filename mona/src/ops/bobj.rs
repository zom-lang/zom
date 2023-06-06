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

pub fn build(mut args: Args) -> Result<ExitStatus, anyhow::Error> {
    // default ouput_file to `output.o`, it's where because with `default_value_t`, that doesn't work.
    args.output_file = if args.emit_ir {
        Some(PathBuf::from(r"output.ll"))
    }else {
        Some(PathBuf::from(r"output.o"))
    };

    println!("{:?}", args);
    todo!()
}
