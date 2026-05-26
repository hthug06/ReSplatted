use clap::Parser;

/// Struct to get all argument from the CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Adress of the target
    #[arg(long)]
    pub address: String,
}
