use std::time::Instant;

use input_reader::InputReader;
pub mod input_reader;
use led_renderer::LedRenderer;
pub mod led_renderer;

const FRAME_TIMING: u128 = 34;

fn main() {
    print!("Startup\n");

    //? setup

    let input_reader = InputReader::new();
    let led_renderer = LedRenderer::new();
    // let dmx_renderer = DmxRenderer::new();

    //? infinite programmloop whose speed is capped by the FRAME_TIMING attribute

    loop {
        let fps_limit_timestamp = Instant::now();
        
        input_reader.process_input("Serial");
        led_renderer.render();
        // dmx_renderer.render();

        print!("{}\n", fps_limit_timestamp.elapsed().as_millis());
        while fps_limit_timestamp.elapsed().as_millis() < FRAME_TIMING {}
    }
}