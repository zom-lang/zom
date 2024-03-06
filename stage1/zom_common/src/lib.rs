//! `zom_common` is related to all things that every Zom crate could want to use, like errors.

pub mod operator;
pub mod token;

/// Return the current commit hash when this crate was compiled
pub fn commit_hash() -> String {
    env!("GIT_HASH").to_string()
}

/// Return the current date when this crate was compiled
pub fn build_date() -> String {
    env!("COMPILED_DATE").to_string()
}

/// Return the target triple when this crate was compiled
pub fn build_target_triple() -> String {
    env!("TARGET_TRIPLE").to_string()
}
