use clap::Parser;
use anyhow::{Result, Context};
use std::{
    fs::{self},
    path::Path,
};
use yokoi::cartridge;

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

    let contents = fs::read(path)
        .with_context(|| format!("Failed to read from file: {:#?}", path))?;

    let cartridge = cartridge::Cartridge::from_bytes(&contents)
        .with_context(|| format!("Failed to parse ROM data into cartridge"))?;

    // println!("{}", contents);
    println!("Cartidge Type: {}", cartridge.get_cart_type()?);
    println!("Licensee Code: {}", cartridge.get_lic_code()?);

    Ok(())
}
