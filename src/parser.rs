use anyhow::Result;
use syn::{File, parse_file};

pub fn parse(path: &str) -> Result<File> {
    let src = std::fs::read_to_string(path)?;
    Ok(parse_file(&src)?)
}
