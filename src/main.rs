use clap::Parser;
use anyhow::{Result, Context};
use std::{
    fs::{self},
    path::Path,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file
    #[arg(short, long)]
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let expanded = shellexpand::tilde(&args.path).to_string();
    let path = Path::new(&expanded);
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read from file: {:#?}", path))?;
    println!("{}", contents);
    Ok(())
}
