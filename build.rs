use std::{fs, path};

fn main() {
    let version_file = path::Path::new("./VERSION");
    if let Ok(version) = fs::read_to_string(version_file) {
        println!("cargo:rustc-env=APP_VERSION={}", version.trim());
    } else {
        eprintln!("Failed to read VERSION file");
    }
}
