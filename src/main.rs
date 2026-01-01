mod cli;
mod parser;
mod types;
mod emitter;
mod imports;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
