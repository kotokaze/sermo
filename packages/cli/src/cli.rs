use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {}

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
  #[arg(short, long, help = "The path to the device file")]
  pub device: PathBuf,

  #[arg(short, long, help = "The baud rate to use")]
  pub baud: u32,

  #[command(subcommand)]
  pub command: Option<Commands>,
}

pub fn parse() -> Cli {
  self::Cli::parse()
}
