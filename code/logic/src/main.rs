#![warn(missing_docs)]
//! The entrypoint for ruhige_waldgeräusche.<br>
//! Please consult the README for in depth documentation

use std::time::Instant;

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
use config_store::GeneralConfigStore;
use config_store::DmxConfigStore;
use config_store::LedConfigStore;
use config_store::InputConfigStore;
use config_store::GlobalVarsStore;

/// logs a message to the predefined file ".log"
pub mod logger;

fn main() {
    //? setup

    let general_config_store = GeneralConfigStore::new();
    let led_config_store = LedConfigStore::new();
    let dmx_config_store = DmxConfigStore::new();
    let input_config_store = InputConfigStore::new();
    let mut global_vars_store = GlobalVarsStore::new();
    let mut input_parser = InputParser::new(&input_config_store);
    let mut led_renderer = LedRenderer::new(&led_config_store);
    let mut scanners = Scanners::new(&dmx_config_store);
    let mut dmx_renderer = DmxRenderer::new();

    //? infinite programmloop whose speed is capped by the FRAME_TIMING attribute

    loop {
        let fps_limit_timestamp = Instant::now();

        match led_renderer.render() {
            Ok(_) => (),
            Err(error) => logger::log(error.as_str())
        };
        match dmx_renderer.render(&scanners) {
            Ok(_) => (),
            Err(error) => logger::log(error.as_str())
        }
        match input_parser.process_input(&mut led_renderer, &mut scanners, &mut dmx_renderer, &mut global_vars_store) {
            Ok(_) => (),
            Err(error) => logger::log(error.as_str())
        };

        print!("Elapsed: {} | Frame timing: {}\n", fps_limit_timestamp.elapsed().as_millis(), general_config_store.get_frame_timing());
        while fps_limit_timestamp.elapsed().as_millis() < general_config_store.get_frame_timing() as u128 {}
    }
}