use std::io::{Read, Write};
use std::sync::Arc;
use std::{thread, time};

use serial2::SerialPort;

fn do_main() -> Result<(), ()> {
	let args: Vec<_> = std::env::args().collect();
	if args.len() != 3 {
		let prog_name = args[0].rsplit_once('/').map(|(_parent, name)| name).unwrap_or(&args[0]);
		eprintln!("Usage: {} PORT BAUD", prog_name);
		return Err(());
	}

	let port_name = &args[1];
	let baud_rate: u32 = args[2]
		.parse()
		.map_err(|_| eprintln!("Error: invalid baud rate: {}", args[2]))?;

	let port = SerialPort::open(&port_name, baud_rate)
		.map_err(|e| eprintln!("Error: Failed to open {}: {}", port_name, e))?;
	let port = Arc::new(port);

	// Spawn a thread to read from stdin and write to the serial port.
	std::thread::spawn({
		let port = port.clone();
		let port_name = port_name.to_owned();
		move || {
			if let Err(()) = read_stdin_loop(port, &port_name) {
				std::process::exit(1);
			}
		}
	});

	// Read from serial port and write to stdout in main thread.
	read_serial_loop(port, port_name)?;

	Ok(())
}

fn read_stdin_loop(port: Arc<SerialPort>, port_name: &str) -> Result<(), ()> {
	let stdin = std::io::stdin();
	let mut stdin = stdin.lock();
	let mut buffer = [0; 512];
	let mut test_buffer = [120; 512];
	test_buffer[0] = 69;
	for i in 1..512 {
		if i % 3 == 0 {
			test_buffer[i] = 200;
		}
		if i % 3 == 1 {
			test_buffer[i] = 100;
		}
		if i % 3 == 2 {
			test_buffer[i] = 10;
		}
	}
	loop {
		let read = stdin
			.read(&mut buffer)
			.map_err(|e| eprintln!("Error: Failed to read from stdin: {}", e))?;
		if read == 0 {
			return Ok(());
		} else {
			port.write(&test_buffer);
			// port.write(&buffer[..read])
			// 	.map_err(|e| eprintln!("Error: Failed to write to {}: {}", port_name, e))?;
		}
	}
}

fn read_serial_loop(port: Arc<SerialPort>, port_name: &str) -> Result<(), ()> {
	let mut buffer: [u8; 512] = [0x00; 512];
	loop {
        //thread::sleep(time::Duration::from_millis(1000));
        //println!("reading...");
		match port.read(&mut buffer) {
			Ok(0) => return Ok(()),
			Ok(mut n) => {
				std::io::stdout()
					.write_all(&buffer[..n])
					.map_err(|e| eprintln!("Error: Failed to write to stdout: {}", e))?;
                print!("{:?}", &buffer[..n]);
			},
			Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
			Err(e) => {
				eprintln!("Error: Failed to read from {}: {}", port_name, e);
				return Err(());
			},
		}
	}
}

fn main() {
	if let Err(()) = do_main() {
		std::process::exit(1);
	}
}