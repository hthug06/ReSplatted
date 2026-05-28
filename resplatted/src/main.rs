use crate::cli::Args;
use crate::client::{core::MinecraftClient, state::ProtocolState};
use clap::Parser;
use log::{LevelFilter, info};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::io::Error;
use std::sync::Arc;

mod cli;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // First of all init log
    // Start log
    TermLogger::init(
        LevelFilter::Info, // Use LevelFilter::Debug for debugging (LOL)
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize logger");

    info!("Starting ReSplatted v{}", env!("CARGO_PKG_VERSION"));

    let args = Args::parse();
    info!("{:?}", args);

    // Configure target
    let target = Arc::new(args.address);
    let port = args.port;
    info!("Connecting to {}:{}...", target, port);

    if args.status {
        // Create a new client and Init the tcp connection
        let mut client = MinecraftClient::connect(&target, port).await?;

        // Status state
        client
            .handshake(&target, port, ProtocolState::Status)
            .await?;

        // fetch the status from the server
        client.fetch_and_display_status(&target).await?;
    } else {
        // All the bots task
        let mut bot_tasks = Vec::new();

        for i in 1..=args.bot_number {
            // Set the name here, used one in function but more simple for logs
            let bot_name = format!("ReSplatted_{}", i);
            let target_ptr = Arc::clone(&target);

            // Start a new background task,
            // This task is an entire bot
            let handle = tokio::spawn(async move {
                // info!("Démarrage du bot {}", bot_name);

                let mut client = match MinecraftClient::connect(&target_ptr, port).await {
                    Ok(c) => c,
                    Err(e) => {
                        log::error!("[{}] Failed to connect: {}", bot_name, e);
                        return; // Stop the background task
                    }
                };

                // Handshake
                if let Err(e) = client
                    .handshake(&target_ptr, port, ProtocolState::Login)
                    .await
                {
                    log::error!("[{}] Failed Handshake: {}", bot_name, e);
                    return;
                }

                // Login
                if let Err(e) = client.login(&bot_name).await {
                    log::error!("[{}] Failed Login: {}", bot_name, e);
                    return;
                }

                // Configuration
                match client.configuration().await {
                    Ok(_) => info!("[{}] Connected !", bot_name),
                    Err(e) => {
                        log::error!("[{}] Failed Configuration: {}", bot_name, e);
                        return;
                    }
                };

                // Play | Game loop
                if let Err(e) = client.enter_game().await {
                    log::error!("[{}] Disconnected: {}", bot_name, e);
                }
            });

            // Add the task to the vec
            bot_tasks.push(handle);
        }

        // Wait here for the bot
        // The program stop when every bot disconnected
        for task in bot_tasks {
            let _ = task.await;
        }
    }

    info!("Disconnecting from {}:{}...", target, port);
    info!("Stopping ReSplatted");
    Ok(())
}
