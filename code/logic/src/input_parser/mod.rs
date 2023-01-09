use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::{DmxRenderer, ColorTransitionMode};
use crate::dmx_renderer::fixture::FixtureType;
use crate::config_store::InputConfigStore;
use crate::logging;

use std::sync::mpsc::{Receiver};

use std::time::Duration;
use std::sync::Arc;
use serial2::SerialPort;
use std::cmp::max;
    
/// The struct to define how the InputParser should look like
pub struct InputParser {
    /// The input usb port
    module_connectors: Vec<Arc<SerialPort>>,
    rx_channel: Receiver<String>
}

/// Responsible for reading and parsing possible input sources
impl InputParser {
    /// This creates, fills and returns the InputParser object
    /// - opens and configures the serial input port
    /// - calculates bpm based on a hardcoded start beat_duration
    pub fn new(input_config_store: &InputConfigStore, connected_modules: Vec<String>, rx_channel: Receiver<String>) -> InputParser {

        let module_connectors = Self::spawn_module_connectors(connected_modules, input_config_store.get_baud_rate());
        
        InputParser {
            module_connectors: module_connectors,
            rx_channel: rx_channel
        }
    }
    /// acts acordingly to the processed input gathered by gather_serial_input()
    pub fn process_input(&mut self, led_renderer: &mut LedRenderer, dmx_renderer: &mut DmxRenderer) -> Result<Vec<u8>, String> {
        let mut input: Vec<u8> = match InputParser::gather_serial_input(self) {
            Ok(e) => e,
            Err(error) => return Err(error)
        };

        match self.gather_rest_input() {
            Ok(e) => e.iter().for_each(|param| { input.push(*param) }),
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
    fn gather_serial_input(&mut self) -> Result<Vec<u8>, String> {
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
    fn gather_rest_input(&mut self) -> Result<Vec<u8>, String> {
        let mut result = vec!();
        return match self.rx_channel.try_recv() {
            Ok(e) => {
                if e.as_bytes().len() > 0 {
                    e
                    .split("&")
                    .for_each(|pair| { 
                        pair
                        .split("=")
                        .for_each(|param| {
                            let mut parsed_param = 0;
                            param
                                .as_bytes()
                                .iter()
                                .rev()
                                .enumerate()
                                .for_each(|(i, byte_param)| {
                                    parsed_param += (byte_param - 48) * max(i *  10, 1) as u8
                                });
                            result.push(parsed_param);
                        })
                    });
                    return Ok(result)
                }
                return Ok(vec!())
            },
            Err(_) => Ok(vec!())
        }
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