use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::{thread, time};

fn main() {
    let mut child = Command::new("python3")
        .arg("python/main.py")
        .stdin(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    
    loop {
        child_stdin.write_all("Hello, world!\n")?;
        thread::sleep(time::Duration::from_millis(1000));
    }
    
}