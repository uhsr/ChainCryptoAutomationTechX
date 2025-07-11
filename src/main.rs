// src/main.rs
/*
 * Main executable for ChainCryptoAutomationTechX
 */

use clap::Parser;
use chaincryptoautomationtechx::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChainCryptoAutomationTechX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
