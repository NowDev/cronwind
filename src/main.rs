mod config;
mod job;
mod runner;
mod logger;
mod utils;

use std::error::Error;
use std::fs::{self, File};
use std::sync::Arc;
use clap::Parser;
use daemonize::Daemonize;
use tokio::sync::watch;
use tokio::runtime::Runtime;

const PID_FILE: &str = "/tmp/cronwind.pid";
const STDOUT_FILE: &str = "/tmp/cronwind.out";
const STDERR_FILE: &str = "/tmp/cronwind.err";

// Args
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    daemon: bool,
}

fn cleanup_pid_file() {
    if let Err(e) = fs::remove_file(PID_FILE) {
        eprintln!("Failed to remove PID file: {}", e);
    }
}

async fn run(shutdown_rx: &mut watch::Receiver<bool>) -> Result<(), Box<dyn Error>> {
    let config_path = "config.json";
    let config = config::load_config(config_path)?;

    if config.jobs.is_empty() {
        log::info!("No jobs defined in config, exiting...");
        return Ok(());
    } else {
        log::info!("Loaded {} jobs from {}", config.jobs.len(), config_path);
    }

    let runner = runner::Runner::new(config).await?;
    
    // Start the runner with shutdown signal
    tokio::select! {
        result = runner.start() => {
            if let Err(e) = result {
                log::error!("Runner error: {}", e);
            }
        }
        _ = shutdown_rx.changed() => {
            log::info!("Shutting down gracefully...");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Shutdown signal channel (so we can 'talk' to the runner)
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
    let shutdown_tx = Arc::new(shutdown_tx);

    if args.daemon {
        let stdout = File::create(STDOUT_FILE)?;
        let stderr = File::create(STDERR_FILE)?;
        
        // docs: https://docs.rs/daemonize/0.5.0/daemonize
        let daemonize = Daemonize::new()
            .pid_file(PID_FILE)
            .chown_pid_file(true)
            .working_directory(".")
            .stdout(stdout)
            .stderr(stderr);

        match daemonize.start() {
            Ok(_) => {
                logger::setup_logging(true);
                log::info!("Running as a daemon. PID file at {}", PID_FILE);
            }
            Err(e) => {
                eprintln!("Error starting daemon: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        logger::setup_logging(false);
    }

    // For graceful shutdowns
    // TODO: Maybe handle jobs that are running when the signal is received
    let tx = Arc::clone(&shutdown_tx);
    ctrlc::set_handler(move || {
        log::info!("Received shutdown signal");
        if args.daemon {
            cleanup_pid_file();
        }
        let _ = tx.send(true);
    })?;

    let runtime = Runtime::new()?;
    runtime.block_on(run(&mut shutdown_rx))?;

    Ok(())
}