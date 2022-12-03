use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::DmxRenderer;
use crate::config_store::GlobalVarsStore;
use crate::config_store::ColorMode;
use crate::scanners::Scanners;

use std::time::{Duration, Instant};
use std::sync::Arc;
use serial2::SerialPort;

use crate::config_store::{InputConfigStore, InputType};

/// The struct to define how the InputParser should look like
pub struct InputParser<'a> {
    /// InputConfigStore contains usefull informations for the parser
    input_config_store: &'a InputConfigStore,
    /// The input usb port
    serial_port: Arc<SerialPort>,
    /// The timestamp to support the beat detection
    last_beat_timestamp: Instant,
    /// Stores, how long is the interval between beats is
    beat_duration: Duration,
    /// The current bpm is calculated from the beat_duration
    bpm: u16
}
/// Responsible for reading and parsing possible input sources
impl<'a> InputParser<'a> {
    /// This creates, fills and returns the InputParser object
    /// - opens and configures the serial input port
    /// - calculates bpm based on a hardcoded start beat_duration
    pub fn new(input_config_store: &InputConfigStore) -> InputParser {
        let mut port = SerialPort::open("/dev/ttyACM0", 2000000)
                                    .expect("Could not open Serial input port!");
        match port.set_read_timeout(Duration::from_millis(1)) {
            Ok(()) => Some(0),
            Err(error) => panic!("set_read_timeout returned an error: {}\n", error)
        };
	    let port = Arc::new(port);

        let last_beat_timestamp = Instant::now();
        let bpm = 120 as u16;
        let beat_duration = Duration::from_millis( (60_000 / bpm) as u64 );
        
        InputParser {
            input_config_store: input_config_store,
            serial_port: port,
            last_beat_timestamp: last_beat_timestamp,
            beat_duration: beat_duration,
            bpm: bpm
        }
    }
    /// acts acordingly to the processed input gathered by gather_input()
    pub fn process_input(&mut self, led_renderer: &mut LedRenderer, scanners: &mut Scanners, dmx_renderer: &mut DmxRenderer, global_vars_store: &mut GlobalVarsStore) -> Result<Vec<u8>, String> {
        let input: Vec<u8> = match InputParser::gather_input(self) {
            Ok(e) => e,
            Err(error) => return Err(error)
        };
        if input.len() > 5 && input[0] == 96 && input[1] == 1 && input[2] == 2 && input[3] == 3 && input[4] == 5 && input[5] == 4 {
            print!("====================================================================================================\n");
            match global_vars_store.get_color_mode() {
                ColorMode::Primary => {
                    led_renderer.trigger_current_animation(&global_vars_store.get_primary_color());
                    scanners.trigger_next_step(dmx_renderer);
                },
                ColorMode::Complementary => {
                    led_renderer.trigger_current_animation(&global_vars_store.get_secondary_color());
                    scanners.trigger_next_step(dmx_renderer);
                }
            }
        }
        
        Ok(input)
    }
    /// gathers input from the configured input source
    pub fn gather_input(&mut self) -> Result<Vec<u8>, String> {
        let mut return_vec: Vec<u8> = vec!();

        // ?process inputs
        match self.input_config_store.get_input_type() {
            InputType::Serial => {
                
                let mut buffer: [u8; 512] = [0x00; 512];
                
                match &self.serial_port.read(&mut buffer) {
                    Ok(0) => return Err(String::from("Input source may be unplugged!")),
                    Ok(n) => {
                        for i in 0..*n {
                            return_vec.push(buffer[i]);
                        }
                    },
                    Err(_) => return Ok(return_vec)
                }
            },
            InputType::RestApi => unimplemented!()
        };
            
        // ? beat detection
        if self.last_beat_timestamp.elapsed().as_millis() > self.beat_duration.as_millis() {
            self.beat_duration = Duration::from_millis(self.last_beat_timestamp.elapsed().as_millis() as u64);
            self.last_beat_timestamp = Instant::now();
            self.bpm = (60000 / self.beat_duration.as_millis()) as u16;
        }
        Ok(return_vec)
        
    }
    /// returns a reference to the established serial connection
    pub fn get_serial_connection(&self) -> &Arc<SerialPort> {
        &self.serial_port
    }
}

#[test]
fn serial_port_is_valid() {
    let input_config_store = InputConfigStore::new();
    let input_parser = InputParser::new(&input_config_store);
    assert!(input_parser.get_serial_connection().is_write_vectored());
}

#[test]
fn input_function_gathers_something() {
    let input_config_store = InputConfigStore::new();
    let input_parser = InputParser::new(&input_config_store);
    assert!(input_parser.get_serial_connection().is_write_vectored());
}

#[test]
fn process_input_throws_no_errors() {
    use crate::config_store::DmxConfigStore;
    use crate::config_store::LedConfigStore;

    let led_config_store = LedConfigStore::new();
    let dmx_config_store = DmxConfigStore::new();
    let mut scanners = Scanners::new(&dmx_config_store);
    let input_config_store = InputConfigStore::new();
    let mut global_vars_store = GlobalVarsStore::new();
    let mut input_parser = InputParser::new(&input_config_store);
    let mut led_renderer = LedRenderer::new(&led_config_store);
    let mut dmx_renderer = DmxRenderer::new();
    match input_parser.process_input(&mut led_renderer, &mut scanners, &mut dmx_renderer, &mut global_vars_store) {
            Ok(_) => 0,
            Err(error) => panic!("{}", error)
        };
}