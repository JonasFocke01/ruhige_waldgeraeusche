 use std::time::Instant;
use std::thread;
pub mod serial;
pub mod write_leds;


//? configuration
const BUTTON_COUNT   : usize = 40;
const LED_STRIP_COUNT: usize =  4;

fn calculate_dmx(button_states: [u8; BUTTON_COUNT], debug: bool) -> [u8; 512] {
    let channel_states: [u8; 512] = [0; 512];

    if debug {
        assert!(false, "Not implemented!");
    }

    // assert!(false, "Not implemented!{}\n", button_states[0]);
    channel_states
}

fn calculate_leds(button_states: [u8; BUTTON_COUNT], debug: bool) -> [[[u8; 3]; 150]; LED_STRIP_COUNT] {
    let pixel_states: [[[u8; 3]; 150]; LED_STRIP_COUNT] = [[[0; 3]; 150]; LED_STRIP_COUNT];

    if debug {
        assert!(false, "Not implemented!");
    }
    // assert!(false, "Not implemented!{}\n", button_states[0]);
    


    pixel_states
}



fn main() {
    print!("Startup\n");
    //? setup

    //* the button states array where 0 represents NOT ACTIVE, 255 represents ACTIVE and 0-255 represent POTENTIOMETER VALUES
    let mut button_states: [u8; BUTTON_COUNT] = [0; BUTTON_COUNT];

    let mut serial_writing_thread;
    // let mut serial_reading_thread;

    let mut dmx_values: [u8; 512] = [0; 512];
    let mut dmx_calculating_thread;

    let mut led_values: [[[u8; 3]; 150]; LED_STRIP_COUNT] = [[[0; 3]; 150]; LED_STRIP_COUNT];
    let mut led_calculating_thread;

    let mut keyboard_input_device = "/dev/ttyUSB0";

    //? infinite programmloop
    print!("Entering programmloop\n");
    let mut loop_counter: u32 = 0;
    while loop_counter < u32::MAX {
        loop_counter += 1;

        let fps_limit_timestamp = Instant::now();
        
        serial_writing_thread  = thread::Builder::new().name(String::from("led writing")).spawn(move || write_leds::write_leds(false));
        //serial_reading_thread  = thread::Builder::new().name(String::from("serial reading")).spawn(move || serial::read_serial(false));
        led_calculating_thread = thread::Builder::new().name(String::from("led calculation")).spawn(move || calculate_leds(button_states, false));
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
        while fps_limit_timestamp.elapsed().as_millis() < 34 {}
    }
}