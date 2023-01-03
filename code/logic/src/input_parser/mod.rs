use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::{DmxRenderer, ColorTransitionMode};
use crate::dmx_renderer::fixture::FixtureType;
use crate::config_store::InputConfigStore;
use crate::logging;

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
    pub fn new(input_config_store: &InputConfigStore, connected_modules: Vec<String>) -> InputParser {

        let module_connectors = Self::spawn_module_connectors(connected_modules, input_config_store.get_baud_rate());
        
        InputParser {
            module_connectors: module_connectors
        }
    }
    /// acts acordingly to the processed input gathered by gather_input()
    pub fn process_input(&mut self, led_renderer: &mut LedRenderer, dmx_renderer: &mut DmxRenderer) -> Result<Vec<u8>, String> {
        let mut input: Vec<u8> = match InputParser::gather_input(self) {
            Ok(e) => e,
            Err(error) => return Err(error)
        };
        
        // ! This computes, how the programm should behave by analysing the given input
        while input.len() >= 2 {
            match input.remove(0) {
                1..=19 => (),
                20 => { // Color to red
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((255.0, 0.0, 0.0), None));
                        led_renderer.set_current_color((255.0, 0.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                21 => { // color to orange
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((255.0, 165.0, 0.0), None));
                        led_renderer.set_current_color((255.0, 165.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                22 => { // color to Purple
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((128.0, 0.0, 128.0), None));
                        led_renderer.set_current_color((128.0, 0.0, 128.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                23 => { // color to blue
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((0.0, 0.0, 255.0), None));
                        led_renderer.set_current_color((0.0, 0.0, 255.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                24 => { // color to green
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((0.0, 255.0, 0.0), None));
                        led_renderer.set_current_color((0.0, 255.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                25 => { // color to yellow
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((255.0, 255.0, 0.0), None));
                        led_renderer.set_current_color((255.0, 255.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                26 => { // color to white
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((255.0, 255.0, 255.0), None));
                        led_renderer.set_current_color((255.0, 255.0, 255.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                27 => { // color to light blue
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((173.0, 216.0, 230.0), None));
                        led_renderer.set_current_color((173.0, 216.0, 230.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                28 => { // color to pink
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((255.0, 182.0, 193.0), None));
                        led_renderer.set_current_color((255.0, 182.0, 193.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                29 => { // color to rainbow
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color(vec!(FixtureType::Scanner), ((0.0, 0.0, 0.0), Some(0)));
                        led_renderer.set_rainbow_mode(true);
                    }
                },
                30 => { // color transition mode
                    if input.remove(0) == 1 {
                        dmx_renderer.set_color_transition_mode(Some(ColorTransitionMode::Animative));
                    } else {
                        dmx_renderer.set_color_transition_mode(Some(ColorTransitionMode::Instant));
                    }
                },
                31 => { // color transition speed change (fader)
                    dmx_renderer.set_color_transition_speed(input.remove(0));
                }
                _ => ()
            }
        }
        
        Ok(input)
    }
    /// gathers input from the configured input source
    /// Todo: (long term) this should happen in a sepparate thread for performance reasons
    pub fn gather_input(&mut self) -> Result<Vec<u8>, String> {
        let mut return_vec: Vec<u8> = vec!();
      
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
    pub fn spawn_module_connectors(connectors_to_spawn: Vec<String>, baud_rate: u64) -> Vec<Arc<SerialPort>> {
        let mut connectors: Vec<Arc<SerialPort>> = vec!();
        for connector in connectors_to_spawn.iter() {
            let mut port = match SerialPort::open(format!("/dev/{}", connector).as_str(), baud_rate as u32) {
                Ok(e) => e,
                Err(_) => {
                    logging::log(format!("Could not open Serial input port {}", connector).as_str(), logging::LogLevel::Warning, true);
                    panic!("Could not open Serial input port");
                }
            };
            match port.set_read_timeout(Duration::from_millis(1)) {
                Ok(()) => Some(0),
                Err(error) => {
                    logging::log(format!("set_read_timeout returned an Error {}", error).as_str(), logging::LogLevel::Warning, true);
                    panic!("set_read_timeout returned an error: {}\n", error);
                }
            };
            connectors.push(Arc::new(port));         
        }
        connectors
    }
}