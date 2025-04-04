#![allow(unused_macros)]
#![allow(unused_imports)]

use spdlog::{
    formatter::{Formatter, FormatterContext},
    prelude::*,
    sink::StdStreamSink,
    Logger, Record, StringBuf,
};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

#[derive(Clone)]
struct ColoredFormatter;

impl Formatter for ColoredFormatter {
    fn format(
        &self,
        record: &Record<'_>,
        dest: &mut StringBuf,
        _ctx: &mut FormatterContext<'_>,
    ) -> spdlog::Result<()> {
        use std::fmt::Write;

        // Define ANSI color codes for log levels
        let color_code = match record.level() {
            Level::Trace => "\x1b[90m",    // Gray
            Level::Debug => "\x1b[36m",    // Cyan
            Level::Info => "\x1b[32m",     // Green
            Level::Warn => "\x1b[33m",     // Yellow
            Level::Error => "\x1b[31m",    // Red
            Level::Critical => "\x1b[35m", // Magenta
        };

        let reset_code = "\x1b[0m"; // Reset color
        let eol = if std::env::consts::OS == "windows" {
            "\r\n"
        } else {
            "\n"
        };

        // Write the formatted log message with colors
        write!(
            dest,
            "{0}[{source}] [{level}] {logger}:{1}{payload}{2}{1}",
            color_code,
            eol,
            reset_code,
            source = record.source_location().map_or("unknown", |loc| loc.file()),
            level = record.level(),
            logger = record.logger_name().unwrap(),
            payload = record.payload()
        )
        .unwrap();

        Ok(())
    }
}

// Define a global static logger
static CLIENT_LOGGER: OnceLock<Arc<Logger>> = OnceLock::new();
static ENGINE_LOGGER: OnceLock<Arc<Logger>> = OnceLock::new();

pub fn get_client_logger() -> &'static Logger {
    CLIENT_LOGGER.get().expect("Logger not initialized")
}
pub fn get_engine_logger() -> &'static Logger {
    ENGINE_LOGGER.get().expect("Logger not initialized")
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let stdout_sink = StdStreamSink::builder();
    let stdout_sink = Arc::new(
        stdout_sink
            .stderr()
            .formatter(Box::new(ColoredFormatter))
            .build()?,
    );

    let new_logger = Logger::builder()
        .name("client")
        .level_filter(LevelFilter::All)
        .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Warn))
        .sink(stdout_sink)
        .build()?;

    let en = Arc::new(new_logger.clone());
    en.set_flush_period(Some(Duration::from_secs(3)));
    // Fork the default logger and configure it

    // Store the new logger in the global static variable
    CLIENT_LOGGER.set(en).expect("Failed to set client logger");

    let mut ci = new_logger.clone();
    let _ = ci.set_name(Some("engine"));
    let ci = Arc::new(ci);
    ci.set_flush_period(Some(Duration::from_secs(3)));
    // Fork the default logger and configure it

    // Store the new logger in the global static variable
    ENGINE_LOGGER.set(ci).expect("Failed to set client logger");

    Ok(())
}

// Internal logging functions
pub fn log_info_internal(logger: &'static Logger, message: &str) {
    spdlog::info!(logger: logger, "{}", message);
}

pub fn log_error_internal(logger: &'static Logger, message: &str) {
    spdlog::error!(logger: logger,"{}", message);
}

pub fn log_debug_internal(logger: &'static Logger, message: &str) {
    spdlog::debug!(logger: logger,"{}", message);
}

pub fn log_trace_internal(logger: &'static Logger, message: &str) {
    spdlog::trace!(logger: logger,"{}", message);
}

pub fn log_warn_internal(logger: &'static Logger, message: &str) {
    spdlog::warn!(logger: logger,"{}", message);
}

//client loging
#[macro_export]
macro_rules! rge_info {
    ($($args:tt)+) => {

        $crate::log::log_info_internal($crate::log::get_client_logger(), &format!($($args)+))
    };
}
#[macro_export]
macro_rules! rge_error {
    ($($args:tt)+) => {
        $crate::log::log_error_internal($crate::log::get_client_logger(), &format!($($args)+))
    };
}
#[macro_export]
macro_rules! rge_warn {
    ($($args:tt)+) => {
        $crate::log::log_warn_internal($crate::log::get_client_logger(), &format!($($args)+))
    };
}

#[macro_export]
macro_rules! rge_trace {
    ($($args:tt)+) => {
        $crate::log::log_trace_internal($crate::log::get_client_logger(), &format!($($args)+))
    };
}
#[macro_export]
macro_rules! rge_critical {
    ($($args:tt)+) => {
        $crate::log::log_critical_internal($crate::log::get_client_logger(), &format!($($args)+))
    };
}
//engine logging
macro_rules! rge_engine_info {
    ($($args:tt)+) => {
        spdlog::info!(logger: $crate::log::get_engine_logger(), $($args)+)
    };
}
pub(crate) use rge_engine_info;
macro_rules! rge_engine_error {
    ($($args:tt)+) => {
        spdlog::error!(logger: $crate::log::get_engine_logger(), $($args)+)
    };
}
pub(crate) use rge_engine_error;
macro_rules! rge_engine_warn {
    ($($args:tt)+) => {
        spdlog::warn!(logger: $crate::log::get_engine_logger(), $($args)+)
    };
}

pub(crate) use rge_engine_warn;
macro_rules! rge_engine_trace {
    ($($args:tt)+) => {
        spdlog::trace!(logger: $crate::log::get_engine_logger(), $($args)+)
    };
}
pub(crate) use rge_engine_trace;
macro_rules! rge_engine_critical {
    ($($args:tt)+) => {
        spdlog::critical!(logger: $crate::log::get_engine_logger(), $($args)+)
    };
}
pub(crate) use rge_engine_critical;
