// https://github.com/exercism/rust/tree/630a56517c9015341cdd96233632eda9ad8f44c4/exercises/concept/semi-structured-logs
// https://exercism.org/tracks/rust/exercises/semi-structured-logs/solutions/mrl5

use std::fmt::Display;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level, message)
}

pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let level_string = match self {
      LogLevel::Info => "INFO",
      LogLevel::Warning => "WARNING",
      LogLevel::Error => "ERROR",
    };
    write!(f, "{}", level_string)
  }
}
