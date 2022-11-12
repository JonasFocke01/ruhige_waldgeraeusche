use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::DmxRenderer;

use std::sync::Arc;
use serial2::SerialPort;

use std::time::Duration;

use crate::config_store::InputConfigStore;

pub struct InputParser<'a> {
    input_config_store: &'a InputConfigStore,
    serial_port: Arc<SerialPort>
}

impl<'a> InputParser<'a> {
    pub fn new(input_config_store: &InputConfigStore) -> InputParser {
        let mut port = SerialPort::open("/dev/ttyUSB0", 2000000).unwrap();
        port.set_read_timeout(Duration::from_millis(1));
	    let port = Arc::new(port);

        
        InputParser {
            input_config_store: input_config_store,
            serial_port: port
        }
    }
    pub fn process_input(&self, led_renderer: &LedRenderer, dmx_renderer: &DmxRenderer, input_type: &str) -> bool {
        println!("Parsing Input for {} buttons", self.input_config_store.get_button_count());

        let input: Vec<u8> = InputParser::gather_input(&self, input_type);
        if input.len() > 0 && input[0] == 96 && input[1] == 1 && input[2] == 2 && input[3] == 3 && input[4] == 5 && input[5] == 4 {
            led_renderer.spawn_snake();
            dmx_renderer.all_up();
        } 
        
        if input.len() > 0 {
            false
        } else {
            true
        }
    }
    pub fn gather_input(&self, input_type: &str) -> Vec<u8> {
        let mut return_vec: Vec<u8> = vec!();
        if input_type == "Serial" {
            println!("Gathering from Serial");

            let mut buffer: [u8; 512] = [0x00; 512];

            match &self.serial_port.read(&mut buffer) {
                Ok(0) => {},
                Ok(n) => {
                    for i in 0..*n {
                        return_vec.push(buffer[i]);
                    }
                }
                Err(_) => {}
            }
        }
        return_vec
        
    }
    pub fn get_serial_connection(&self) -> &Arc<SerialPort> {
        &self.serial_port
    }
}

#[test]
fn serial_port_valid() {
    let input_config_store = InputConfigStore::new();
    let input_parser = InputParser::new(&input_config_store);
    assert!(input_parser.get_serial_connection().is_write_vectored());
}