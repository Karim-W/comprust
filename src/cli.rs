// use clap::Parser;
use std::path::PathBuf;
// #[derive(Parser)]
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser,Debug)]
pub struct Cli {
    pub source: PathBuf,
    pub destination: PathBuf,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
