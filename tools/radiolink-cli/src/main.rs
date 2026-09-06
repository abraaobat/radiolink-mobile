use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "radiolink", version, about = "RadioLink Platform CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scan for compatible radio/TNC devices.
    Scan,
    /// Monitor KISS/AX.25 traffic.
    Monitor,
    /// Show APRS-related commands (placeholder).
    Aprs,
    /// Show Packet-related commands (placeholder).
    Packet,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Scan => println!("RadioLink scan: transport adapters not implemented yet"),
        Command::Monitor => println!("RadioLink monitor: KISS backend not connected yet"),
        Command::Aprs => println!("RadioLink APRS module bootstrap"),
        Command::Packet => println!("RadioLink Packet module bootstrap"),
    }

    Ok(())
}
