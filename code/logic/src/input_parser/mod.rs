use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::DmxRenderer;

use std::sync::Arc;
use serial2::SerialPort;

use std::time::{Duration, Instant};

use crate::config_store::InputConfigStore;

pub struct InputParser<'a> {
    input_config_store: &'a InputConfigStore,
    serial_port: Arc<SerialPort>,
    last_beat_timestamp: Instant,
    beat_duration: Duration,
    bpm: u8
}

impl<'a> InputParser<'a> {
    pub fn new(input_config_store: &InputConfigStore) -> InputParser {
        let mut port = SerialPort::open("/dev/ttyUSB0", 2000000).unwrap();
        port.set_read_timeout(Duration::from_millis(1));
	    let port = Arc::new(port);

        let last_beat_timestamp = Instant::now();
        let beat_duration = Duration::from_millis(500);
        let bpm = 120;
        
        InputParser {
            input_config_store: input_config_store,
            serial_port: port,
            last_beat_timestamp: last_beat_timestamp,
            beat_duration: beat_duration,
            bpm: bpm
        }
    }
    pub fn process_input(&mut self, led_renderer: &LedRenderer, dmx_renderer: &DmxRenderer, input_type: &str) -> bool {
        println!("Parsing Input for {} buttons", self.input_config_store.get_button_count());

        let input: Vec<u8> = InputParser::gather_input(self, input_type);
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
    pub fn gather_input(&mut self, input_type: &str) -> Vec<u8> {
        let mut return_vec: Vec<u8> = vec!();

        // ?process inputs
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

        // ?beat detection
        if self.last_beat_timestamp.elapsed().as_millis() > self.beat_duration.as_millis() {
            self.beat_duration = Duration::from_millis(self.last_beat_timestamp.elapsed().as_millis() as u64);
            self.last_beat_timestamp = Instant::now();
            self.bpm = (60000 / self.beat_duration.as_millis()) as u8;
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