use crate::cli::{Args, WaitingMode};
use crate::client::{core::MinecraftClient, state::ProtocolState};
use clap::Parser;
use log::{LevelFilter, info, warn};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::io::Error;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
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
            let delay_ms = match args.waiting_mode {
                // Linear: Each bot wait a little bit more than the previous one
                // Bot 0 wait 0ms, Bot 2 wait 50ms, Bot 3 attend 100ms...
                WaitingMode::Linear => i as u64 * args.wait,

                // Static: EVERY bot wait the samt time and connect at the same time after this delay
                WaitingMode::Static => args.wait,

                // Random: Each bot wait a random time before connecting
                WaitingMode::Random => {
                    if args.wait > 0 {
                        rand::random_range(1..args.wait)
                    } else {
                        0
                    }
                }
            };

            // Set the name here, used one in function but more simple for logs
            let bot_name = format!("{}_{}", args.name, i);
            let target_ptr = Arc::clone(&target);

            // For log
            let count_ptr = Arc::clone(&connected_count);

            // To disconnect bots
            let child_token = cancel_token.clone();

            // To send message
            let message = args.message.clone();

            // Start a new background task,
            // This task is an entire bot
            let handle = tokio::spawn(async move {
                // In the waiting, check if the program stopped
                if delay_ms > 0 {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_millis(delay_ms)) => {}
                        _ = child_token.cancelled() => return,
                    }
                }

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
                    result = client.enter_game(message) => {
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
                        // *10 else server can't send the last packet they have
                        tokio::time::sleep(Duration::from_millis((i*10) as u64)).await;
                        // Close the TCP Socket nicely
                        if let Err(e) = client.disconnect().await {
                            warn!("[{}] Error during disconnecting: {}", bot_name, e);
                        }

                        let current = count_ptr.fetch_sub(1, Ordering::SeqCst) - 1;
                        info!("[{}] Stopped by user. {}/{}", bot_name, current, total_bots);
                    }
                }
            });

            // Add the task to the vec
            bot_tasks.push(handle);
        }

        let mut wait_all_bots = tokio::spawn(async move {
            for task in bot_tasks {
                let _ = task.await;
            }
        });

        tokio::select! {
            // CTRL+C
            _ = tokio::signal::ctrl_c() => {
                info!("Disconnecting all bots from {}:{}...", target, port);
                // Stop all task with the cancellation token
                cancel_token.cancel();
                // Wait for bots to disconnect
                let _ = wait_all_bots.await;
            }
            // All bot disconnected naturally
            _ = &mut wait_all_bots => {
                info!("All bots are disconnected or the test is finished.");
            }
        }
    }

    info!("Stopping ReSplatted");
    Ok(())
}
