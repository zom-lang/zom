pub use inkwell::targets::TargetMachine;

/// This function is an abstraction for the `mona` bin.
pub fn get_target_triple() -> String {
    TargetMachine::get_default_triple().to_string()
}
