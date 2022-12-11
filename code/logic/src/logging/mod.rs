use std::fs;
use std::io::Write;
use chrono;
use colored::*;

/// describes the priority level at which the messages should be logged
pub enum LogLevel {
    /// level-1
    Info,
    /// level-2
    Warning,
    /// level-3
    Debug
}

const LOG_LEVEL: LogLevel = LogLevel::Info; // Todo: evaluate if it is worth it to make this to a consolearg

/// This loggs a given message with the given prioritylevel to the console and to a logfile if persist is true.
pub fn log(message: &str, log_level: LogLevel, persist: bool) {
    let datetime = format!("{}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S"));
    let mut file: fs::File = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(".log")
                .unwrap();

    match LOG_LEVEL {
        LogLevel::Warning =>  
            match log_level {
                LogLevel::Warning => {
                    print!("{} {}\n", "Warn:".bright_red(), message.bright_white());
                    if persist { writeln!(&mut file, "{} - {}", datetime, format!("Warn : {}", message)).unwrap(); }
                },
                _ => return
            },
        LogLevel::Info =>  
            match log_level {
                LogLevel::Info => {
                    print!("{} {}\n", "Info:".blue(), message);
                    if persist { writeln!(&mut file, "{} - {}", datetime, format!("Info : {}", message)).unwrap() };
                },
                LogLevel::Warning => {
                    print!("{} {}\n", "Warn:".bright_red(), message.bright_white());
                    if persist { writeln!(&mut file, "{} - {}", datetime, format!("Warn : {}", message)).unwrap() };
                },
                _ => return
            },
        LogLevel::Debug =>  
            match log_level {
                LogLevel::Debug => {
                    print!("{} {}\n", "Debug:".purple(), message);
                    if persist { writeln!(&mut file, "{} - {}", datetime, format!("Debug: {}", message)).unwrap() };
                },
                _ => return
            }
    }
}