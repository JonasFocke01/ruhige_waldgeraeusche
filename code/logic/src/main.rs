#![warn(missing_docs)]
//! The entrypoint for ruhige_waldgeräusche.<br>
//! Please consult the README for in depth documentation

use std::time::Instant;
use std::sync::Arc;
use serial2::SerialPort;
use std::cmp;

/// Responsible for reading and parsing possible input sources
pub mod input_parser;
use input_parser::InputParser;
/// Responsible for storing and rendering the current pixel state 
pub mod led_renderer;
use led_renderer::LedRenderer;
/// Responsible for collecting and rendering all values from dmx devices
pub mod dmx_renderer;
use dmx_renderer::DmxRenderer;
/// This is responsible for storing the current scanner state and how they should react to certain situations
pub mod scanners;
use scanners::Scanners;

/// holding all stores that load and provide configurations or global variables
pub mod config_store;
use config_store::DmxConfigStore;
use config_store::LedConfigStore;
use config_store::InputConfigStore;
use config_store::GlobalVarsStore;

/// logs a message to the predefined file ".log"
pub mod logging;

/// This describes the to connected Arduino type
pub enum ArduinoModule {
    /// describes as a dmx adapter
    DmxAdapter,
    /// describes as an input device
    Input
}

fn main() {
    //? setup

    let led_config_store = LedConfigStore::new();
    let frame_timing = led_config_store.get_frame_timing();
    let dmx_config_store = DmxConfigStore::new();
    let input_config_store = InputConfigStore::new();
    let mut global_vars_store = GlobalVarsStore::new();
    let mut led_renderer = LedRenderer::new(&led_config_store);
    let mut scanners = Scanners::new(&dmx_config_store);
    
    let input_port_matches: Vec<(ArduinoModule, String)> = map_serial_connections_to_arduino_modules(&input_config_store);
    let mut input_port_paths: Vec<String> = vec!();
    let mut dmx_adapter_port: &str = "";
    
    for e in input_port_matches.iter() {  
        match e.0 {
            ArduinoModule::DmxAdapter => {
                dmx_adapter_port = &e.1;
            },
            ArduinoModule::Input => input_port_paths.push(e.1.to_string())
        }
    }
    let mut input_parser = InputParser::new(&input_config_store, input_port_paths);
    let mut dmx_renderer = DmxRenderer::new(&input_config_store, dmx_adapter_port);

    //? infinite programmloop whose speed is capped by the FRAME_TIMING attribute

    let mut truncate_peak_ms: u128 = 0;
    let mut truncate_index: u16 = 0;
    let mut fps_limit_timestamp = Instant::now();
    loop {
        fps_limit_timestamp = Instant::now();

        match led_renderer.render() {
            Ok(_) => (),
            Err(error) => logging::log(error.as_str(), logging::LogLevel::Warning, true)
        };
        match dmx_renderer.render(&scanners) {
            Ok(_) => (),
            Err(error) => logging::log(error.as_str(), logging::LogLevel::Warning, true)
        }
        match input_parser.process_input(&mut led_renderer, &mut scanners, &mut dmx_renderer, &mut global_vars_store) {
            Ok(_) => (),
            Err(error) => logging::log(error.as_str(), logging::LogLevel::Warning, true)
        };

        if truncate_index == 6000 {
            let mut log_level = logging::LogLevel::Info;
            let mut persist = false;
            if truncate_peak_ms > frame_timing.into() {
                log_level = logging::LogLevel::Warning;
                persist = true;
            }
            scanners.trigger_next_step(&mut dmx_renderer);
            logging::log(format!("Peak elapsed ms since last log: {}", truncate_peak_ms.to_string()).as_str(), log_level, persist);
            truncate_peak_ms = 0;
            truncate_index = 0;
        } else {
            truncate_peak_ms = cmp::max(truncate_peak_ms, fps_limit_timestamp.elapsed().as_millis());
            truncate_index += 1;
        }
        while fps_limit_timestamp.elapsed().as_millis() < 1 {  } //This is to not totaly run at max speed and fry the processor
    }
}

fn map_serial_connections_to_arduino_modules(input_config_store: &InputConfigStore) -> Vec<(ArduinoModule, String)> {
    let to_be_checked_ports: Vec<String> = input_config_store.get_input_ports().to_vec();
    let mut mapped_inputs = vec!();
    for port in to_be_checked_ports.iter() {
        match map_serial_connection_to_arduino_modules(port.to_string(), input_config_store) {
            Ok(input_module) => mapped_inputs.push((input_module, port.to_string())),
            Err(e) => logging::log(e.as_str(), logging::LogLevel::Warning, true)
        }
    }
    for port in mapped_inputs.iter() {
        match port.0 {
            ArduinoModule::Input => logging::log(format!("mapped port {} to Input", port.1).as_str(), logging::LogLevel::Info, false),
            ArduinoModule::DmxAdapter => logging::log(format!("mapped port {} to DmxAdapter", port.1).as_str(), logging::LogLevel::Info, false)
        }
    }
    mapped_inputs
}

fn map_serial_connection_to_arduino_modules(serial_port_path: String, input_config_store: &InputConfigStore) -> Result<ArduinoModule, String> {
    let port = match SerialPort::open(format!("/dev/{}", serial_port_path).as_str(), input_config_store.get_baud_rate() as u32) {
        Ok(e) => e,
        Err(_) => {
            return Err(format!("Could not open Serial input port {}", serial_port_path));
        }
    };
    let port = Arc::new(port);
    let mut buffer: [u8; 512] = [0x00; 512];
    let mut return_vec = vec!();
    match port.read(&mut buffer) {
        Ok(0) => return Err("Input source may be unplugged!".to_string()),
        Ok(n) => {
            for i in 0..n {
                return_vec.push(buffer[i]);
            }
        },
        Err(e) => return Err(format!("Error while reading serial port: {} {}", serial_port_path, e))
    };
    if return_vec.len() < 1 { return Err("No input found".to_string()); }

    // print!("{:?}\n", return_vec);
    if return_vec[0] == 69 { //Todo: this should be 1
        return Ok(ArduinoModule::DmxAdapter)
    } else if return_vec[0] == 2 {
        return Ok(ArduinoModule::Input)
    } else {
        return Err(format!("Correct input device for {} could not be determined\n", serial_port_path))
    }
}