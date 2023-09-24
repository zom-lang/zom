pub use inkwell::targets::TargetMachine;

/// This function is an abstraction for the `zomc` bin.
#[inline]
pub fn get_target_triple() -> String {
    TargetMachine::get_default_triple()
        .as_str()
        .to_owned()
        .to_str()
        .expect("Error while trying to get the target trimple to a &str.")
        .to_owned()
}
