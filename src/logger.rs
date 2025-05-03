use fern::Dispatch;
use log::LevelFilter;
use chrono::Local;

pub fn setup_logging() {
    let _ = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
            "[{}][{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("cronwind.log").unwrap())
        .apply();
}