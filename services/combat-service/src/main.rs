//! Combat Service
//!
//! Microservice for combat mechanics and damage calculation.

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value = "8082")]
    port: u16,
    
    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(args.log_level)
        .init();
    
    info!("Starting Combat Service on port {}", args.port);
    
    // TODO: Initialize combat-core services
    // TODO: Start HTTP server
    // TODO: Start gRPC server
    
    info!("Combat Service started successfully");
    
    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Combat Service");
    
    Ok(())
}
