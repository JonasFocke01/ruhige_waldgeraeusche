use std::time::Instant;
use std::thread;

use std::io::{self, BufRead};
use rand::Rng;
use std::fs;

pub mod serial;
pub mod write_leds;

fn calculate_dmx(button_states: [u8; BUTTON_COUNT], debug: bool) -> [u8; 512] {
    let channel_states: [u8; 512] = [0; 512];

    if debug {
        assert!(false, "Not implemented!");
    }

    // assert!(false, "Not implemented!{}\n", button_states[0]);
    channel_states
}

fn calculate_leds(button_states: [u8; BUTTON_COUNT], led_strip_count: u8, debug: bool) -> [[[u8; 3]; 150]; led_strip_count] {
    let pixel_states: [[[u8; 3]; 150]; led_strip_count] = [[[0; 3]; 150]; led_strip_count];

    if debug {
        assert!(false, "Not implemented!");
    }
    // assert!(false, "Not implemented!{}\n", button_states[0]);
    


    pixel_states
}



fn main() {
    print!("Startup\n");

    let BUTTON_COUNT: u8 = 20;

    //? configuration
    let file = fs::File::open("config.json")
                .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
                .expect("file should be proper JSON");
    let leds = json.get("leds")
                .expect("file should have leds key");

    let frame_timing = &leds["frame_timing"];
    let led_strip_count = &leds["led_strip_count"];

    //? setup

    //* the button states array where 0 represents NOT ACTIVE, 255 represents ACTIVE and 0-255 represent POTENTIOMETER VALUES
    let mut button_states: [u8; BUTTON_COUNT] = [0; BUTTON_COUNT];

    let mut serial_writing_thread;
    // let mut serial_reading_thread;

    let mut dmx_values: [u8; 512] = [0; 512];
    let mut dmx_calculating_thread;

    let mut led_values: [[[u8; 3]; 150]; led_strip_count] = [[[0; 3]; 150]; led_strip_count];
    let mut led_calculating_thread;

    let mut keyboard_input_device = "/dev/ttyUSB0";

    //? infinite programmloop
    print!("Entering programmloop\n");
    loop {
        let fps_limit_timestamp = Instant::now();
        
        serial_writing_thread  = thread::Builder::new().name(String::from("led writing")).spawn(move || write_leds::write_leds(false));
        //serial_reading_thread  = thread::Builder::new().name(String::from("serial reading")).spawn(move || serial::read_serial(false));
        led_calculating_thread = thread::Builder::new().name(String::from("led calculation")).spawn(move || calculate_leds(button_states, led_strip_count, false));
        dmx_calculating_thread = thread::Builder::new().name(String::from("dmx calculation")).spawn(move || calculate_dmx(button_states, false));
        
        //let button_states_buffer = serial_reading_thread.unwrap().join().unwrap();
        // if button_states_buffer.0 {
        //     button_states = button_states_buffer.1;
        // }
        dmx_values = dmx_calculating_thread.unwrap().join().unwrap();
        led_values = led_calculating_thread.unwrap().join().unwrap();
        let _dummy_value = serial_writing_thread.unwrap().join();

        //wait until 33 ms passed to limit to 30 fps
        print!("{}\n", fps_limit_timestamp.elapsed().as_millis());
        while fps_limit_timestamp.elapsed().as_millis() < frame_timing {}
    }
}