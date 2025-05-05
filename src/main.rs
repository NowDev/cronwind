mod config;
mod job;
mod runner;
mod logger;

use clap::Parser;
use std::error::Error;

/// Args
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    daemon: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    logger::setup_logging();

    if args.daemon {
        // Here goes the logic to run as daemon (systemd or manual background)
        log::info!("Running as daemon...");
    } else {
        log::info!("Running normally...");
    }

    let config_path = "config.json";
    let config = config::load_config(config_path)?;

    if config.jobs.is_empty() {
        log::info!("No jobs defined in config, exiting...");
        return Ok(());
    }

    let runner = runner::Runner::new(config).await?;
    runner.start().await?;

    Ok(())
}