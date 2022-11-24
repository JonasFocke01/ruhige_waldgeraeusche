use std::io::{self, Write, BufRead};
use std::process::{Command, Stdio};
use std::{thread, time};
use rand::Rng;
use std::fs;

fn main() {
    print!("ENTERING!\n");
    let stdin = io::stdin();
    
    let file = fs::File::open("config.json")
    .expect("file should open read only");
    
    let json: serde_json::Value = serde_json::from_reader(file)
                .expect("file should be proper JSON");
                
                let leds = json.get("leds")
                .expect("file should have leds key");
                
                let led_pin = &leds["led_pin"];
    print!("LED_PIN: {:?}", led_pin);

    let mut child = Command::new("python3")
        .arg("python/main.py")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdin = child.stdin.as_mut().unwrap();
    let mut rng = rand::thread_rng();
    loop {
        let line1 = stdin.lock().lines().next().unwrap().unwrap();
        let r = rng.gen_range(0..200);
        let g = rng.gen_range(0..200);
        let b = rng.gen_range(0..200);
        println!("{} {} {}\n", r, g, b);
        println!("{}", line1);
        child_stdin.write_all(format!("{} {} {}\n", line1, 0, 0).as_bytes());
        thread::sleep(time::Duration::from_millis(1000));
    }
    
}