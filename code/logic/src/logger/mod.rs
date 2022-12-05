use std::fs;
use std::io::{Write};
use chrono;

/// logs a message to the file ".log" (yes, no name in fromt of the .)
/// prepends the current date and time to make the log more readable
pub fn log(message: &str) {
    let datetime = format!("{}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S"));
    let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(".log")
                    .unwrap();

    writeln!(&mut file, "{} - {}", datetime, message).unwrap();
}