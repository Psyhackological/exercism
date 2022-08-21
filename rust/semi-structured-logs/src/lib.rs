// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    use LogLevel::*;
    match level {
        Info => info(message),
        Warning => warn(message),
        Error => error(message),
        Debug => debug(message),
    }
}

// format! macro produces a String, not a &str, hence we do not to use to_owned or String::from()
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}

pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}

pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}

pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {}", message)
}
