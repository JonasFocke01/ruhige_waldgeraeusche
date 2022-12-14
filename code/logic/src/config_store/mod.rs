use crate::logging;
use std::fs::File;

/// Enums all possible input types
pub enum InputType {
    /// This gathers input from serial
    Serial,
    /// This gathers input from a rest api
    RestApi
}
/// Enums all possible color modes
pub enum ColorMode {
    /// primary color mode
    Primary,
    /// complementary color mode
    Complementary
}
/// Enums all possible movement modes
pub enum MovementMode {
    /// synchronized movement
    Synchronized,
    /// asynchronized movement
    Asynchronized
}
/// The struct to define how the InputConfigStore should look like
pub struct InputConfigStore {
    input_ports: Vec<String>,
    baud_rate: u64
}
/// The struct to define how the LedConfigStore should look like
pub struct LedConfigStore {
    led_count_per_strip: u64,
    strip_count: u64,
    led_pin: u64,
    led_freq_hz: u64,
    led_dma: u64,
    led_brightness: u64,
    led_invert: u64,
    led_channel: u64,
    pixel_offset: u64,
    parameter_count: u64,
    frame_timing: u64
}
/// The struct to define how the DmxConfigStore should look like
pub struct DmxConfigStore {
    scanner_count: u8,
    scanner_animation_count: u64
}
/// The struct to define how the GlobalVarsStore should look like
pub struct GlobalVarsStore {
    primary_color: (f32, f32, f32),
    secondary_color: (f32, f32, f32),
    color_mode: ColorMode,
    movement_mode: MovementMode
}
/// Loads and defines configurations related to led strips
impl LedConfigStore {
    /// This creates, fills and returns a LedConfigStore object
    pub fn new() -> LedConfigStore {
        let file = match File::open("config.json") {
            Ok(e) => e,
            Err(_) => {
                logging::log("config file should open read only", logging::LogLevel::Warning, true);
                panic!("config file should open read only");
            }
        };

        let json: serde_json::Value = match serde_json::from_reader(file) {
            Ok(e) => e,
            Err(_) => {
                logging::log("config file should be proper JSON", logging::LogLevel::Warning, true);
                panic!("config file should be proper JSON");
            }
        };

        let leds = match json.get("leds") {
            Some(e) => e,
            None => {
                logging::log("config file does not contain 'leds' key", logging::LogLevel::Warning, true);
                panic!("config file does not contain 'leds' key");
            }
        };

        //Todo: this has no need to be in the config store
        let parameter_count = 6;

        //Todo: cleanup config store
        LedConfigStore {
            led_count_per_strip: leds["led_count_per_strip"].as_u64().expect("config file does not contain expected sub key leds/led_count_per_strip"),
            strip_count: leds["strip_count"].as_u64().expect("config file does not contain expected sub key leds/strip_count"),
            led_pin: leds["led_pin"].as_u64().expect("config file does not contain expected sub key leds/led_pin"),
            led_freq_hz: leds["led_freq_hz"].as_u64().expect("config file does not contain expected sub key leds/led_freq_hz"),
            led_dma: leds["led_dma"].as_u64().expect("config file does not contain expected sub key leds/led_dma"),
            led_brightness: leds["led_brightness"].as_u64().expect("config file does not contain expected sub key leds/led_brightness"),
            led_invert: leds["led_invert"].as_u64().expect("config file does not contain expected sub key leds/led_invert"),
            led_channel: leds["led_channel"].as_u64().expect("config file does not contain expected sub key leds/led_channel"),
            pixel_offset: leds["pixel_offset"].as_u64().expect("config file does not contain expected sub key leds/pixel_offset"),
            frame_timing: leds["frame_timing"].as_u64().expect("config file does not contain expected sub key leds/frame_timing"),
            parameter_count: parameter_count
        }
    }
    /// Returns led_count_per_strip
    pub fn get_led_count_per_strip(&self) -> u64 {
        self.led_count_per_strip
    }
    /// Returns strip_count
    pub fn get_strip_count(&self) -> u64 {
        self.strip_count
    }
    /// Returns led_pin
    pub fn get_led_pin(&self) -> u64 {
        self.led_pin
    }
    /// Returns led_freq_hz
    pub fn get_led_freq_hz(&self) -> u64 {
        self.led_freq_hz
    }
    /// Returns led_dma
    pub fn get_led_dma(&self) -> u64 {
        self.led_dma
    }
    /// Returns led_brightness
    pub fn get_led_brightness(&self) -> u64 {
        self.led_brightness
    }
    /// Returns led_invert
    pub fn get_led_invert(&self) -> u64 {
        self.led_invert
    }
    /// Returns led_channel
    pub fn get_led_channel(&self) -> u64 {
        self.led_channel
    }
    /// Returns pixel_offset
    pub fn get_pixel_offset(&self) -> u64 {
        self.pixel_offset
    }
    /// Returns parameter_count
    pub fn get_parameter_count(&self) -> u64 {
        self.parameter_count
    }
    /// Returns frame_timing
    pub fn get_frame_timing(&self) -> u64 {
        self.frame_timing
    }
}

/// Loads and defines configurations related to the input
impl InputConfigStore {
    /// This creates, fills and returns an InputConfigStore object
    pub fn new() -> InputConfigStore {
        let file = match File::open("config.json") {
            Ok(e) => e,
            Err(_) => {
                logging::log("config file should open read only", logging::LogLevel::Warning, true);
                panic!("config file should open read only");
            }
        };

        let json: serde_json::Value = match serde_json::from_reader(file) {
            Ok(e) => e,
            Err(_) => {
                logging::log("config file should be proper JSON", logging::LogLevel::Warning, true);
                panic!("config file should be proper JSON");
            }
        };

        let input = match json.get("input") {
            Some(e) => e,
            None => {
                logging::log("config file does not contain 'input' key", logging::LogLevel::Warning, true);
                panic!("config file does not contain 'input' key");
            }
        };

        let input_ports_key = match input["input_ports"].as_array() {
            Some(e) => e,
            None => panic!("config does not contain key input/input_ports (array needed)")
        };

        let mut input_ports: Vec<String> = vec!();
        for e in input_ports_key.iter() {
            input_ports.push(e.as_str().expect("could not convert input/input_ports[i] as_str").to_string());
        }

        InputConfigStore {
            input_ports: input_ports,
            baud_rate: input["baud_rate"].as_u64().expect("config file does not contain expected sub key input/baud_rate"),
        }
    }
    /// Returns button_count
    pub fn get_input_ports(&self) -> &Vec<String> {
        &self.input_ports
    }
    pub fn get_baud_rate(&self) -> u64 {
        self.baud_rate
    }
}

/// Loads and defines configurations related to dmx
impl DmxConfigStore {
    /// This creates, fills and returns a DmxConfigStore object
    pub fn new() -> DmxConfigStore {
        let scanner_count = 5;

        let file = match File::open("config.json") {
            Ok(e) => e,
            Err(_) => {
                logging::log("config file should open read only", logging::LogLevel::Warning, true);
                panic!("config file should open read only");
            }
        };

        let json: serde_json::Value = match serde_json::from_reader(file) {
            Ok(e) => e,
            Err(_) => {
                logging::log("config file should be proper JSON", logging::LogLevel::Warning, true);
                panic!("config file should be proper JSON");
            }
        };

        let dmx = match json.get("dmx") {
            Some(e) => e,
            None => {
                logging::log("config file does not contain 'dmx' key", logging::LogLevel::Warning, true);
                panic!("config file does not contain 'dmx' key");
            }
        };

        DmxConfigStore {
            scanner_count: scanner_count,
            scanner_animation_count: dmx["scanner_animation_count"].as_u64().expect("config file does not contain expected sub key dmx/scanner_animation_count")
        }
    }
    /// Returns scanner_count
    pub fn get_scanner_count(&self) -> u8 {
        self.scanner_count
    }
    /// Returns the scanner_animation_count
    pub fn get_scanner_animation_count(&self) -> u64 {
        self.scanner_animation_count
    }
}

/// Defines global variables
/// This store can also hold utility functions
impl GlobalVarsStore {
    /// This creates, fills and returns a GlobalVarsStore object
    pub fn new() -> GlobalVarsStore {
        let primary_color = (239.0, 2.0, 235.0);
        let secondary_color = (255.0, 0.0, 0.0);

        let color_mode = ColorMode::Primary;

        let movement_mode = MovementMode::Synchronized;

        GlobalVarsStore {
            primary_color: primary_color,
            secondary_color: secondary_color,
            color_mode: color_mode,
            movement_mode: movement_mode
        }
    }
    /// maps a value from range to range <br>
    pub fn map_range(number: f64, from_range: (f64, f64), to_range: (f64, f64)) -> f64 {
        to_range.0 + (number - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
    }
    /// Returns primary_color
    pub fn get_primary_color(&self) -> &(f32, f32, f32){
        &self.primary_color
    }
    /// Sets the primary_color
    pub fn set_primary_color(&mut self, new_primary_color: (f32, f32, f32)) {
        self.primary_color = new_primary_color;
    }
    /// Returns secondary_color
    pub fn get_secondary_color(&self) -> &(f32, f32, f32){
        &self.secondary_color
    }
    /// Sets the secondary_color
    pub fn set_secondary_color(&mut self, new_secondary_color: (f32, f32, f32)) {
        self.secondary_color = new_secondary_color;
    }
    /// Returns color_mode
    pub fn get_color_mode(&self) -> &ColorMode {
        &self.color_mode
    }
    /// Sets the color_mode variable <br>
    /// toggles if new_color_mode: None
    pub fn set_color_mode(&mut self, new_color_mode: Option<ColorMode>) {
        match new_color_mode {
            Some(e) => self.color_mode = e,
            None => self.color_mode = match self.color_mode {
                                        ColorMode::Primary => ColorMode::Complementary,
                                        ColorMode::Complementary => ColorMode::Primary
                                    }
        }
    }
    /// Returns movement_mode
    pub fn get_movement_mode(&self) -> &MovementMode {
        &self.movement_mode
    }
    /// Sets the movement_mode varibale <br>
    /// toggles if movement_mode: None
    pub fn set_movement_mode(&mut self, new_movement_mode: Option<MovementMode>) {
        match new_movement_mode {
            Some(e) => self.movement_mode = e,
            None => self.movement_mode = match self.movement_mode {
                                            MovementMode::Asynchronized => MovementMode::Synchronized,
                                            MovementMode::Synchronized => MovementMode::Asynchronized
                                        }
        }
    }
}