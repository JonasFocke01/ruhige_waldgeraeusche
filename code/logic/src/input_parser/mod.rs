use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::{DmxRenderer, ColorTransitionMode};
use crate::dmx_renderer::fixture::{FixtureType, AnimationType};
use crate::config_store::InputConfigStore;
use crate::logging;

use std::sync::mpsc::{Receiver};

use std::time::Duration;
use std::sync::Arc;
use serial2::SerialPort;

/// Toggles if incomming request should fill the queue or if the queue should be flushed
#[derive(PartialEq)]
pub enum QueueMode {
    /// The queue should be flushed
    Flush,
    /// The queue should be filled
    Queue
}

/// The struct to define how the InputParser should look like
pub struct InputParser {
    /// The input usb port
    module_connectors: Vec<Arc<SerialPort>>,
    /// The channel where this thread receives values from the rest thread
    rx_channel: Receiver<String>,
    /// Fills up if QueueMode is set to Queue and flushes its content if QueueMode is set to Flush
    queue: Vec<u8>,
    /// Controlls whether queue fills up or flushes
    queue_mode: QueueMode,
    /// Stores all fixture types that are changeable by the inputstream
    selected_fixtures: Vec<FixtureType>
}

/// Responsible for reading and parsing possible input sources
impl InputParser {
    /// This creates, fills and returns the InputParser object <br>
    /// - opens and configures the serial input port <br>
    /// - calculates bpm based on a hardcoded start beat_duration
    pub fn new(input_config_store: &InputConfigStore, connected_modules: Vec<String>, rx_channel: Receiver<String>) -> InputParser {

        let module_connectors = Self::spawn_module_connectors(connected_modules, input_config_store.get_baud_rate());

        // Todo: change this to an empty vec as soon as selection buttons are implemented
        let selected_fixtures = vec!(FixtureType::Scanner);
        
        InputParser {
            module_connectors: module_connectors,
            rx_channel: rx_channel,
            queue: vec!(),
            queue_mode: QueueMode::Flush,
            selected_fixtures: selected_fixtures
        }
    }
    /// acts acordingly to the processed input gathered by gather_serial_input()
    pub fn process_input(&mut self, led_renderer: &mut LedRenderer, dmx_renderer: &mut DmxRenderer) -> Result<Vec<u8>, String> {
        let mut input: Vec<u8> = match InputParser::gather_serial_input(self) {
            Ok(e) => e,
            Err(error) => return Err(error)
        };

        match self.gather_rest_input() {
            // Todo: evaluate if this is better suited
            // Ok(e) => e.append(&mut e),
            Ok(e) => e.iter().for_each(|param| { input.push(*param) }),
            Err(error) => return Err(error)
        };

        if self.queue_mode == QueueMode::Flush {
            input.append(&mut self.queue);
        }
        
        // ! This computes, how the programm should behave by analysing the given input
        while input.len() >= 2 {
            let key = input.remove(0);
            let value = input.remove(0);
            match key {
                1..=14 => (),

                // ? color
                20 => { // red
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((255.0, 0.0, 0.0), None));
                        led_renderer.set_current_color((255.0, 0.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                21 => { // orange
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((255.0, 165.0, 0.0), None));
                        led_renderer.set_current_color((255.0, 165.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                22 => { // Purple
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((128.0, 0.0, 128.0), None));
                        led_renderer.set_current_color((128.0, 0.0, 128.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                23 => { // blue
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((0.0, 0.0, 255.0), None));
                        led_renderer.set_current_color((0.0, 0.0, 255.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                24 => { // green
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((0.0, 255.0, 0.0), None));
                        led_renderer.set_current_color((0.0, 255.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                25 => { // yellow
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((255.0, 255.0, 0.0), None));
                        led_renderer.set_current_color((255.0, 255.0, 0.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                26 => { // white
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((255.0, 255.0, 255.0), None));
                        led_renderer.set_current_color((255.0, 255.0, 255.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                27 => { // light blue
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((173.0, 216.0, 230.0), None));
                        led_renderer.set_current_color((173.0, 216.0, 230.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                28 => { // pink
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((255.0, 182.0, 193.0), None));
                        led_renderer.set_current_color((255.0, 182.0, 193.0));
                        led_renderer.set_rainbow_mode(false);
                    }
                },
                29 => { // rainbow
                    if value == 1 && self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color(&self.selected_fixtures, ((0.0, 0.0, 0.0), Some(0)));
                        led_renderer.set_rainbow_mode(true);
                    }
                },
                30 => { // color transition mode
                    if self.queue_mode_allows_continue(key, value){
                        match value {
                            1 => dmx_renderer.set_color_transition_mode(Some(ColorTransitionMode::Animative)),
                            0 => dmx_renderer.set_color_transition_mode(Some(ColorTransitionMode::Instant)),
                            _ => ()
                        }
                    }
                },
                31 => { // color transition speed change (fader)
                    if self.queue_mode_allows_continue(key, value) {
                        dmx_renderer.set_color_transition_speed(value);
                    }
                },

                // ? quickanimations
                190 => {
                    match value {
                        1 => {
                            dmx_renderer.reset_quickanimation_position_index(Some(false));
                            dmx_renderer.set_animation(&self.selected_fixtures, AnimationType::Quickanimation, "square".to_string())
                        },
                        0 => {
                            if self.queue_mode_allows_continue(key, value) {
                                dmx_renderer.reset_quickanimation_position_index(Some(true));
                            }
                        },
                        _ => ()
                    }
                }

                255 => { // Space bar - queue key
                    match value {
                        1 => {
                            self.queue_mode = QueueMode::Queue;
                        },
                        0 => {
                            self.queue_mode = QueueMode::Flush;
                        },
                        _ => ()
                    }
                },
                
                // ! Testroutes
                15 => {
                    let value = value;
                    if self.queue_mode == QueueMode::Queue {
                        self.queue.push(15);
                        self.queue.push(value);
                    } else if value == 1 {
                        dmx_renderer.set_animation(&self.selected_fixtures, AnimationType::Quickanimation, "square".to_string())
                    }

                },
                16 => {
                    ()
                },
                17 => {
                    ()
                },
                18 => {
                    ()
                },
                19 => {
                    ()
                },
                // ! Testroutes

                _ => (/*Todo: log unexpected input */)
            }
        }
        
        Ok(input)
    }
    /// pushes key and value to the queue and returns false if QueueMode is set to Queue <br>
    /// returns true otherwise
    fn queue_mode_allows_continue(&mut self, key: u8, value: u8) -> bool {
        match self.queue_mode {
            QueueMode::Queue => {
                self.queue.append(&mut vec!(key, value));
                false
            },
            QueueMode::Flush => {
                true
            }
        }
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
                                    parsed_param += (byte_param - 48) * 10u32.pow(i as u32) as u8
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