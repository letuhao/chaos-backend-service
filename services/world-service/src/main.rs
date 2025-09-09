//! World Service
//!
//! Microservice for world state and zone management.

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value = "8083")]
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
    
    info!("Starting World Service on port {}", args.port);
    
    // TODO: Initialize world-core services
    // TODO: Start HTTP server
    // TODO: Start gRPC server
    
    info!("World Service started successfully");
    
    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down World Service");
    
    Ok(())
}
