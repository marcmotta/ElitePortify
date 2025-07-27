// src/main.rs
/*
 * Main executable for ElitePortify
 */

use clap::Parser;
use eliteportify::{Result, run};

#[derive(Parser)]
#[command(version, about = "ElitePortify - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
