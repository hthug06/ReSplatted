use crate::cli::Args;
use clap::Parser;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod cli;

fn main() {
    // First of all init log
    // Start log
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize logger");

    info!("Starting ReSplatted v{}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();
    info!("{:?}", args);
}
