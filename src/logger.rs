// RustyNet colored logger.


use log::{Record, Level, Metadata, LevelFilter};
use colored::*;


// Colored logger.
pub struct ColoredLogger;
static LOGGER: ColoredLogger = ColoredLogger;

impl ColoredLogger {

    // Initialize the logger with specified logging level.
    pub fn init(filter: LevelFilter) {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(filter);
    }
}

impl log::Log for ColoredLogger {

    // Enabled for all levels.
    fn enabled(&self, _: &Metadata) -> bool { true }

    // Red for error message, yellow for debug message, and no special format
    // for info message.
    fn log(&self, record: &Record) {
        match record.metadata().level() {
            Level::Error => println!("{} {}", "[ERROR]".red().bold(),
                                     record.args()),
            Level::Info  => println!("{} {}", "[Rusty]".yellow().bold(),
                                     record.args()),
            Level::Debug => println!("{} {}", "[DEBUG]".blue().bold(),
                                     record.args()),
            _ => panic!("Logging level not supported."),
        }
    }

    // Nothing to flush.
    fn flush(&self) {}
}