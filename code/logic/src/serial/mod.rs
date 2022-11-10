use serialport::{available_ports, SerialPortType};
use std::process::Command;
use std::io::{self, Write};
use std::time::Duration;

const BUTTON_COUNT   : usize = 40;
const LED_STRIP_COUNT: usize =  4;

pub fn read_serial(debug: bool) -> (bool, [u8; BUTTON_COUNT]) {
    //print!("TEST123");
    let mut parsed_button_states: [u8; BUTTON_COUNT] = [0; BUTTON_COUNT];
    let mut serial_input: [u8; BUTTON_COUNT * 2] = [0; BUTTON_COUNT * 2];
    let mut startbit_received = false;
    let port = serialport::new("/dev/ttyUSB0", 2000000)
                .timeout(Duration::from_secs(2))
                .open();

    // let output = Command::new("cu")
    //                     .arg("-l")
    //                     .arg("/dev/ttyUSB0")
    //                     .arg("-s")
    //                     .arg("2000000")
    //                     .output()
    //                     .expect("FEHLER FÜR DEN COMMAND");

    // let output = Command::new("ls")
    //                         .arg("-la")
    //                         .output()
    //                         .expect("TESTFEHLER");


    // println!("status: {:?}", output.status());
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
                port.read(serial_buf.as_mut_slice());
                print!("{:?}", serial_buf);
                if serial_buf[0] == 105 {
                    startbit_received = true;
                }
                for index in serial_buf.iter() {
                    // if *index != 0 {
                    //     print!("{:?}\n", index);
                    //     print!("HIT_____________________________________\n");
                    // }
                    // print!("ERGEBNIS {:?}\n", index);
                }
                print!("-");
                // ::std::process::exit(1);
        }
        Err(e) => {
            print!("port not okay");
            //::std::process::exit(1);
        }
    }
        
    if debug {
        for (button, index) in parsed_button_states.iter().enumerate(){
            print!("Button {} ({}): {}\n", index, "NO LABEL", button);
        }
    }
    (startbit_received, parsed_button_states)
}

// fn find_device(device_to_find: String) -> Option<String> {

//     const VERBOSE: bool = false;

//     match available_ports() {
//         Ok(ports) => {
//             if VERBOSE {
//                 match ports.len() {
//                     0 => println!("No ports found."),
//                     1 => println!("Found 1 port:"),
//                     n => println!("Found {} ports:", n),
//                 };
//             }
//             for p in ports {
//                 if VERBOSE {
//                     println!("  {}", p.port_name);
//                 }
//                 match p.port_type {
//                     SerialPortType::UsbPort(info) => {
//                         if info.vid == 6790 {
//                             return Some(p.port_name);
//                         }
//                         if VERBOSE {
//                             println!("    Type: USB");
//                             println!("    VID:{} PID:{}", info.vid, info.pid);
//                             println!(
//                                 "     Serial Number: {}",
//                                 info.serial_number.as_ref().map_or("", String::as_str)
//                             );
//                             println!(
//                                 "      Manufacturer: {}",
//                                 info.manufacturer.as_ref().map_or("", String::as_str)
//                             );
//                             println!(
//                                 "           Product: {}",
//                                 info.product.as_ref().map_or("", String::as_str)
//                             );
//                         }
//                     }
//                     SerialPortType::BluetoothPort => {
//                         if VERBOSE {
//                             println!("    Type: Bluetooth");
//                         }
//                     }
//                     SerialPortType::PciPort => {
//                         if VERBOSE {
//                             println!("    Type: PCI");
//                         }
//                     }
//                     SerialPortType::Unknown => {
//                         if VERBOSE {
//                             println!("    Type: Unknown");
//                         }
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             eprintln!("{:?}", e);
//             eprintln!("Error listing serial ports");
//         }
//     }
//     None
    
// }