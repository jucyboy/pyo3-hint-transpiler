use clap::Parser;
use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
pub struct Args {
    #[arg(short = 'i')]
    pub input: Option<String>,

    #[arg(short = 'o')]
    pub output: Option<String>,

    #[arg(short = 'n')]
    pub name: Option<String>
}

#[derive(Deserialize, Default)]
struct FileConfig {
    input: Option<String>,
    output: Option<String>,
    name: Option<String>
}

#[derive(Deserialize, Default)]
struct Config {
    config: Option<FileConfig>
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    let file_cfg = load_config();

    let input = args.input
        .or(file_cfg.input)
        .unwrap_or("src/lib.rs".into());

    let output_dir = args.output
        .or(file_cfg.output)
        .unwrap_or(".".into());

    let name = args.name
        .or(file_cfg.name)
        .unwrap_or(load_package_name()?);

    let ast = crate::parser::parse(&input)?;
    let content = crate::emitter::emit(ast)?;

    let out_path = format!("{}/{}.pyi", output_dir, name);
    fs::write(out_path, content)?;

    Ok(())
}

fn load_config() -> FileConfig {
    let raw = fs::read_to_string("pyo3ht.toml").ok();
    raw.and_then(|s| toml::from_str::<Config>(&s).ok())
        .and_then(|c| c.config)
        .unwrap_or_default()
}

fn load_package_name() -> Result<String> {
    let raw = fs::read_to_string("Cargo.toml")?;
    let value: toml::Value = toml::from_str(&raw)?;
    Ok(value["package"]["name"].as_str().unwrap().to_string())
}
