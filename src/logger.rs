use fern::Dispatch;
use log::LevelFilter;
use chrono::Local;
use std::env;
use std::path::PathBuf;

pub fn setup_logging(is_daemon: bool) {
    let log_file = if is_daemon {
        // When daemonized, we use the home directory
        let mut log_dir = if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".cronwind")
        } else {
            PathBuf::from(".").join(".cronwind")
        };

        if let Err(e) = std::fs::create_dir_all(&log_dir) {
            eprintln!("[!] We couldn't create the log directory: {}", e);
            // Fallback to current dir
            log_dir = PathBuf::from(".");
        }

        log_dir.join("cronwind.log").to_string_lossy().to_string()
    } else {
        "cronwind.log".to_string()
    };

    let dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(&log_file).unwrap_or_else(|e| {
            eprintln!("[!] Failed to open log file {}: {}", log_file, e);
            std::process::exit(1);
        }));

    // Only chain stdout (console) in foreground mode
    let dispatch = if !is_daemon {
        dispatch.chain(std::io::stdout())
    } else {
        dispatch
    };

    if let Err(e) = dispatch.apply() {
        eprintln!("[!] Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    if is_daemon {
        // Since logging isn't set up yet, we use eprintln now.
        eprintln!("[i] Daemon logs will be written to: {}", log_file);
    }
}