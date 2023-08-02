use std::{process::Command, error::Error};
use std::str;
use chrono;


fn get_target() -> Result<String, String> {
    let output = match Command::new("rustc")
        .arg("-vV")
        .output()
        {
            Ok(out) => out,
            Err(err) => return Err(err.to_string())
        };
    let output = match str::from_utf8(&output.stdout) {
        Ok(s) => s,
        Err(err) => return Err(err.to_string())
    };

    let field = "host: ";
    let host = output
        .lines()
        .find(|l| l.starts_with(field))
        .map(|l| &l[field.len()..])
        .ok_or_else(|| {
            format!(
                "`rustc -vV` didn't have a line for `{}`, got:\n{}",
                field.trim(),
                output
            )
        })?.to_string();
    Ok(host)
}

fn get_commit_hast() -> String {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn get_current_date() -> String {
    chrono::offset::Local::now().date_naive().to_string()
}

fn main() -> Result<(), &'static dyn Error>{
    println!("cargo:rustc-env=GIT_HASH={}", get_commit_hast());
    println!("cargo:rustc-env=TARGET_TRIPLE={}", get_target().expect("An error occurs while trying to get the target triple."));
    println!("cargo:rustc-env=COMPILED_DATE={}", get_current_date());
    Ok(())
}