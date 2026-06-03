use clap::{Parser, ValueEnum};
use std::sync::Arc;

/// Struct to get all argument from the CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(help = "The address of the server. Can either be an ip or a domain name")]
    #[arg(long)]
    pub address: String,

    #[clap(help = "The number of bot you want to send to the server")]
    #[arg(long, short, default_value_t = 1)]
    pub bot_number: u32,

    #[clap(help = "The message every bot will spam every second. ")]
    #[arg(long, short)]
    pub message: Option<Arc<String>>,

    #[clap(help = "The prefix for the bots' usernames")]
    #[arg(long, short, default_value = "ReSplatted")]
    pub name: String,

    #[clap(help = "The port of the server")]
    #[arg(long, short, default_value_t = 25565)]
    pub port: u16,

    #[clap(
        help = "if true, display the infos about the minecraft server (like in the multiplayer list of a minecraft client)"
    )]
    #[arg(long, short, default_value_t = false)]
    pub status: bool,

    #[clap(
        help = "The base waiting time (in ms). Used differently depending on the waiting_mode."
    )]
    #[arg(long, short, default_value_t = 1)]
    pub wait: u64,

    #[clap(
        help = "Waiting mode between each bot connection: \n- Linear: delays each bot by (index * wait). \n- Static: every bot waits exactly 'wait' ms. \n- Random: every bot waits a random time between 0 and 'wait'.\n"
    )]
    #[clap(value_enum, long, default_value_t = WaitingMode::Linear)]
    pub waiting_mode: WaitingMode,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum WaitingMode {
    Linear,
    Static,
    Random,
}
