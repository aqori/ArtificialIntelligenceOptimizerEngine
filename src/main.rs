// src/main.rs
/*
 * Main executable for ArtificialIntelligenceOptimizerEngine
 */

use clap::Parser;
use artificialintelligenceoptimizerengine::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceOptimizerEngine - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
