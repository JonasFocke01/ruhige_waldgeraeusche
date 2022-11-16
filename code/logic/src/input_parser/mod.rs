use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::DmxRenderer;
use crate::config_store::GlobalVarsStore;
use crate::config_store::ColorMode;

use std::sync::Arc;
use serial2::SerialPort;

use std::time::{Duration, Instant};

use crate::config_store::{InputConfigStore, InputType};

pub struct InputParser<'a> {
    input_config_store: &'a InputConfigStore,
    serial_port: Arc<SerialPort>,
    last_beat_timestamp: Instant,
    beat_duration: Duration,
    bpm: u8
}

impl<'a> InputParser<'a> {
    pub fn new(input_config_store: &InputConfigStore) -> InputParser {
        let mut port = SerialPort::open("/dev/ttyUSB0", 2000000)
                                    .expect("Could not open Serial input port!");
        match port.set_read_timeout(Duration::from_millis(1)) {
            Ok(()) => Some(0),
            Err(error) => panic!("set_read_timeout returned an error: {}\n", error)
        };
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
    pub fn process_input(&mut self, led_renderer: &mut LedRenderer, dmx_renderer: &mut DmxRenderer, global_vars_store: &mut GlobalVarsStore) -> bool {
        // println!("Parsing Input for {} buttons", self.input_config_store.get_button_count());

        let input: Vec<u8> = InputParser::gather_input(self);
        if input.len() > 0 && input[0] == 96 && input[1] == 1 && input[2] == 2 && input[3] == 3 && input[4] == 5 && input[5] == 4 {
            match global_vars_store.get_color_mode() {
                ColorMode::Primary => led_renderer.spawn_snake(&global_vars_store.get_primary_color()),
                ColorMode::Complementary => led_renderer.spawn_snake(&global_vars_store.get_secondary_color())
            }
            dmx_renderer.all_up();
        }
        
        if input.len() > 0 {
            false
        } else {
            true
        }
    }
    pub fn gather_input(&mut self) -> Vec<u8> {
        let mut return_vec: Vec<u8> = vec!();

        // ?process inputs
        match self.input_config_store.get_input_type() {
            InputType::Serial => {
                
                let mut buffer: [u8; 512] = [0x00; 512];
                
                match &self.serial_port.read(&mut buffer) {
                    Ok(0) => panic!("Input source may be unplugged!"),
                    Ok(n) => {
                        for i in 0..*n {
                            return_vec.push(buffer[i]);
                        }
                    }
                    Err(_) => {}
                }
            },
            InputType::RestApi => unimplemented!()
        };
            
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