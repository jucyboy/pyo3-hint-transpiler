use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile != "release" {
        return;
    }

    // Specify your binary name manually if CARGO_BIN_NAME is missing
    let target_name = env::var("CARGO_BIN_NAME").unwrap_or_else(|_| "pyo3-hint-transpiler".to_string());

    let mut bin_path = PathBuf::from("target").join("release").join(&target_name);
    if cfg!(windows) {
        bin_path.set_extension("exe");
    }

    if !bin_path.exists() {
        eprintln!("Warning: release binary not found at {:?}", bin_path);
        return;
    }

    let mut out_path = PathBuf::from(&target_name);
    out_path.set_extension(bin_path.extension().unwrap());

    match fs::copy(&bin_path, &out_path) {
        Ok(_) => println!("Copied {:?} to {:?}", bin_path, out_path),
        Err(e) => eprintln!("Failed to copy binary: {}", e),
    }
}
