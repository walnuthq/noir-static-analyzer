use clap::Parser;
use std::path::PathBuf;

/// CLI arguments for the Noir Analyzer.
#[derive(Debug, Parser)]
#[command(
    name = "noir-analyzer",
    version = "0.1.0",
    about = "Static analysis for Noir programs"
)]
struct Cli {
    /// Path to the Nargo.toml file
    #[arg(long, value_name = "PATH", default_value = "Nargo.toml")]
    manifest_path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Using manifest path: {:?}", args.manifest_path);
}
