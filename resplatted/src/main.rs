use crate::cli::Args;
use crate::client::core::MinecraftClient;
use crate::client::state::ProtocolState;
use clap::Parser;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::io::{Error, ErrorKind};

mod cli;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    // Configure target
    let target_ip = &args.address;
    let port = args.port;
    let address = format!("{}:{}", target_ip, port);
    info!("Connecting to {}...", address);

    // Init the tcp connection
    let mut client = MinecraftClient::connect(&address).await?;

    if args.status {
        // Status state
        client
            .handshake(target_ip, port, ProtocolState::Status)
            .await?;

        // fetch the status from the server
        client.fetch_and_display_status(target_ip).await?;
    } else {
        // First handshake
        client
            .handshake(target_ip, port, ProtocolState::Login)
            .await?;

        // Then login
        let next_state = match client.login("ReSplatted").await {
            Ok(next_state) => {
                info!("Login state completed, next state is : {:?}", next_state);
                next_state
            }
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Login failed: {}", e),
                ));
            }
        };
    }

    info!("Disconnecting from {}...", address);
    info!("Stopping ReSplatted");
    Ok(())
}
