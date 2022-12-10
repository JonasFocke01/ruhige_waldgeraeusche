use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::DmxRenderer;
use crate::config_store::GlobalVarsStore;
use crate::config_store::ColorMode;
use crate::scanners::Scanners;
use crate::logger;

use std::time::Duration;
use std::sync::Arc;
use serial2::SerialPort;
    
/// The struct to define how the InputParser should look like
pub struct InputParser {
    /// The input usb port
    module_connectors: Vec<Arc<SerialPort>>,
}

/// Responsible for reading and parsing possible input sources
impl InputParser {
    /// This creates, fills and returns the InputParser object
    /// - opens and configures the serial input port
    /// - calculates bpm based on a hardcoded start beat_duration
    pub fn new(connected_modules: Vec<String>) -> InputParser {

        let module_connectors = Self::spawn_module_connectors(connected_modules);
        
        InputParser {
            module_connectors: module_connectors
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
                
        for port in self.module_connectors.iter() {
            let mut buffer: [u8; 512] = [0x00; 512];

            match port.read(&mut buffer) {
                Ok(0) => return Err(String::from("Input source may be unplugged!")),
                Ok(n) => {
                    for i in 0..n {
                        return_vec.push(buffer[i]);
                    }
                },
                Err(_) => return Ok(return_vec)
            }
        }
        
        Ok(return_vec)
    }
    /// This spawns and returns all available connectors
    pub fn spawn_module_connectors(connectors_to_spawn: Vec<String>) -> Vec<Arc<SerialPort>> {
        let mut connectors: Vec<Arc<SerialPort>> = vec!();
        for connector in connectors_to_spawn.iter() {
            let mut port = match SerialPort::open(format!("/dev/{}", connector).as_str(), 115200) {
                Ok(e) => e,
                Err(_) => {
                    logger::log("Could not open Serial input port");
                    panic!("Could not open Serial input port");
                }
            };
            match port.set_read_timeout(Duration::from_millis(1)) {
                Ok(()) => Some(0),
                Err(error) => {
                    logger::log("set_read_timeout returned an Error");
                    panic!("set_read_timeout returned an error: {}\n", error);
                }
            };
            connectors.push(Arc::new(port));         
        }
        connectors
    }
}