// pagi-twin/src/main.rs
//
// Modular Monolith Switchboard — unified entry point for PAGI Twin ecosystem.
// Provides subcommands: web, cli, tui, desktop, daemon
//
// Phase 29: Single binary with multiple operational modes.

use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[command(name = "pagi-twin")]
#[command(about = "PAGI Twin — Unified AGI Desktop Companion", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web server with telemetry services
    Web {
        /// Override the bind address (default: from PHOENIX_WEB_BIND or 127.0.0.1:8888)
        #[arg(short, long)]
        bind: Option<String>,
    },
    /// Interactive CLI mode (future implementation)
    Cli,
    /// Terminal UI mode (future implementation)
    Tui,
    /// Launch desktop GUI (Tauri window)
    Desktop,
    /// Run as background daemon
    Daemon,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    let dotenv_path = pagi_utils::load_dotenv_best_effort();
    
    // Initialize tracing
    pagi_utils::init_tracing();

    let phoenix_name = pagi_utils::env_nonempty("PHOENIX_NAME")
        .unwrap_or_else(|| "Sola".to_string());
    let user_name = pagi_utils::env_nonempty("USER_NAME")
        .unwrap_or_else(|| "User".to_string());

    if let Some(path) = dotenv_path {
        info!("Loaded .env from: {}", path.display());
    }
    info!("Phoenix Name: {}", phoenix_name);
    info!("User Name: {}", user_name);

    let cli = Cli::parse();

    match cli.command {
        Commands::Web { bind } => {
            info!("Starting PAGI Twin in Web Server mode");
            
            // Override bind address if provided
            if let Some(bind_addr) = bind {
                std::env::set_var("PHOENIX_WEB_BIND", bind_addr);
            }

            // Spawn telemetry services as background tasks
            let collector_handle = tokio::spawn(async {
                info!("Starting Vital Pulse Collector (telemetry ingestion)");
                if let Err(e) = run_vital_pulse_collector().await {
                    tracing::error!("Vital Pulse Collector error: {}", e);
                }
            });

            let distributor_handle = tokio::spawn(async {
                info!("Starting Synaptic Pulse Distributor (config updates)");
                if let Err(e) = run_synaptic_pulse_distributor().await {
                    tracing::error!("Synaptic Pulse Distributor error: {}", e);
                }
            });

            // Run the main web server (this will block until shutdown)
            info!("Starting Phoenix Web Server");
            let web_result = phoenix_web::run_server().await;

            // If web server exits, cancel background tasks
            collector_handle.abort();
            distributor_handle.abort();

            web_result?;
        }
        Commands::Cli => {
            info!("Starting PAGI Twin in CLI mode");
            println!("CLI mode not yet implemented. Use 'pagi-twin web' to start the web server.");
            println!("Future: Interactive command-line interface for direct AGI interaction.");
        }
        Commands::Tui => {
            info!("Starting PAGI Twin in TUI mode");
            println!("TUI mode not yet implemented. Use 'pagi-twin web' to start the web server.");
            println!("Future: Terminal-based UI with panels for chat, memory, and system status.");
        }
        Commands::Desktop => {
            info!("Starting PAGI Twin in Desktop mode");
            println!("Desktop mode not yet implemented. Use 'pagi-twin web' to start the web server.");
            println!("Future: Launch Tauri desktop window with full GUI.");
            println!("Note: Desktop frontend is in phoenix-desktop-tauri directory.");
        }
        Commands::Daemon => {
            info!("Starting PAGI Twin in Daemon mode");
            println!("Daemon mode not yet implemented. Use 'pagi-twin web' to start the web server.");
            println!("Future: Run as background service with no UI, API-only mode.");
        }
    }

    Ok(())
}

/// Run the Vital Pulse Collector service (telemetry ingestion)
async fn run_vital_pulse_collector() -> std::io::Result<()> {
    // For now, we'll call the main function from vital_pulse_collector
    // In the future, this should be refactored to a library function
    // that we can call directly
    
    // Placeholder: The actual implementation will need vital_pulse_collector
    // to expose a run() function similar to phoenix-web
    info!("Vital Pulse Collector: Starting on port 8889 (placeholder)");
    
    // TODO: Call vital_pulse_collector::run() once it's converted to a library
    // For now, just keep the task alive
    tokio::time::sleep(tokio::time::Duration::from_secs(u64::MAX)).await;
    
    Ok(())
}

/// Run the Synaptic Pulse Distributor service (config updates via WebSocket)
async fn run_synaptic_pulse_distributor() -> std::io::Result<()> {
    // For now, we'll call the main function from synaptic_pulse_distributor
    // In the future, this should be refactored to a library function
    
    // Placeholder: The actual implementation will need synaptic_pulse_distributor
    // to expose a run() function similar to phoenix-web
    info!("Synaptic Pulse Distributor: Starting on port 8890 (placeholder)");
    
    // TODO: Call synaptic_pulse_distributor::run() once it's converted to a library
    // For now, just keep the task alive
    tokio::time::sleep(tokio::time::Duration::from_secs(u64::MAX)).await;
    
    Ok(())
}
