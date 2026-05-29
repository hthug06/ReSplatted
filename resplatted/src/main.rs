use crate::cli::Args;
use crate::client::{core::MinecraftClient, state::ProtocolState};
use clap::Parser;
use log::{LevelFilter, info, warn};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::io::Error;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio_util::sync::CancellationToken;

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

        // For logs
        let connected_count = Arc::new(AtomicUsize::new(0));
        let total_bots = args.bot_number;

        // Used to disconnect bots
        let cancel_token = CancellationToken::new();

        for i in 1..=args.bot_number {
            // Set the name here, used one in function but more simple for logs
            let bot_name = format!("ReSplatted_{}", i);
            let target_ptr = Arc::clone(&target);

            // For log
            let count_ptr = Arc::clone(&connected_count);

            // To disconnect bots
            let child_token = cancel_token.clone();

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
                    Ok(_) => {
                        let current = count_ptr.fetch_add(1, Ordering::Relaxed) + 1;

                        info!("[{}] Connected ! {}/{}", bot_name, current, total_bots);
                    }
                    Err(e) => {
                        log::error!("[{}] Failed Configuration: {}", bot_name, e);
                        return;
                    }
                };

                // Play | Game loop with ctrl+C handling
                tokio::select! {
                    // Activate if enter_game return an error (disconnected by the server or crashed)
                    result = client.enter_game() => {
                        let current = count_ptr.fetch_sub(1, Ordering::Relaxed) - 1;

                        if let Err(e) = result {
                            log::error!(
                                "[{}] Disconnected: {}. {}/{}",
                                bot_name,
                                e,
                                current,
                                total_bots
                            );
                        }

                        // Stop the program if no bot remain
                        if current == 0 {
                            info!("All bot are disconnected.");
                            info!("Stopping ReSplatted");
                            exit(1);
                        }
                    }

                    // When the user stop the program (ctrl+C)
                    _ = child_token.cancelled() => {

                        // Close the TCP Socket nicely
                        if let Err(e) = client.disconnect().await {
                            warn!("[{}] Error during disconnecting: {}", bot_name, e);
                        }

                        // On décrémente aussi le compteur ici, car le bot s'en va
                        let current = count_ptr.fetch_sub(1, Ordering::SeqCst) - 1;
                        info!("[{}] Stopped by user. {}/{}", bot_name, current, total_bots);
                    }
                }
            });

            // Add the task to the vec
            bot_tasks.push(handle);
        }

        // Ctrl +C handling
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                info!("Disconnecting all bots from {}:{}...", target, port);
                // Stop all task with the cancellation token
                cancel_token.cancel();
            }
            Err(e) => log::error!("Failed to handle Ctrl+C: {}", e),
        }

        // Wait here for the bot
        // Used for log when ctrl+C is called
        for task in bot_tasks {
            let _ = task.await;
        }
    }

    info!("Stopping ReSplatted");
    Ok(())
}
