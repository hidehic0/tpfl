mod load_config;
mod types;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    name: String,
    #[arg(short = 'o', long = "output")]
    output_path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
}
