use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

pub fn init_logger() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
    debug!("Logger initialized");
}
