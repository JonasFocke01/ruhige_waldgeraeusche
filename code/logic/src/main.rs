use std::time::Instant;

use input_parser::InputParser;
pub mod input_parser;
use led_renderer::LedRenderer;
pub mod led_renderer;
use dmx_renderer::DmxRenderer;
pub mod dmx_renderer;
use scanner::Scanner;
pub mod scanner;

use config_store::GeneralConfigStore;
use config_store::DmxConfigStore;
use config_store::LedConfigStore;
use config_store::InputConfigStore;
use config_store::GlobalVarsStore;
pub mod config_store;

fn main() {
    //? setup

    let general_config_store = GeneralConfigStore::new();
    let led_config_store = LedConfigStore::new();
    let dmx_config_store = DmxConfigStore::new();
    let input_config_store = InputConfigStore::new();
    let mut global_vars_store = GlobalVarsStore::new();
    let mut input_parser = InputParser::new(&input_config_store);
    let mut led_renderer = LedRenderer::new(&led_config_store);
    let mut scanner = Scanner::new(&dmx_config_store);
    let mut dmx_renderer = DmxRenderer::new();

    //? infinite programmloop whose speed is capped by the FRAME_TIMING attribute

    loop {
        let fps_limit_timestamp = Instant::now();
        
        match led_renderer.render() {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        };
        match dmx_renderer.render(&scanner) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        }
        match input_parser.process_input(&mut led_renderer, &mut scanner, &mut dmx_renderer, &mut global_vars_store) {
            Ok(_) => (),
            Err(error) => panic!("{}", error)
        };

        print!("Elapsed: {} | Frame timing: {}\n", fps_limit_timestamp.elapsed().as_millis(), general_config_store.get_frame_timing());
        while fps_limit_timestamp.elapsed().as_millis() < general_config_store.get_frame_timing() as u128 {}
    }
}