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
    pixel_offset: u64,
    frame_timing: u64
}
/// The struct to define how the DmxConfigStore should look like
pub struct DmxConfigStore {
    dmx_fixtures: Vec<String>,
    animations: Vec<String>,
    quickanimations: Vec<String>
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

        LedConfigStore {
            led_count_per_strip: leds["led_count_per_strip"].as_u64().expect("config file does not contain expected sub key leds/led_count_per_strip"),
            strip_count: leds["strip_count"].as_u64().expect("config file does not contain expected sub key leds/strip_count"),
            pixel_offset: leds["pixel_offset"].as_u64().expect("config file does not contain expected sub key leds/pixel_offset"),
            frame_timing: leds["frame_timing"].as_u64().expect("config file does not contain expected sub key leds/frame_timing")
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
    /// Returns pixel_offset
    pub fn get_pixel_offset(&self) -> u64 {
        self.pixel_offset
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
    /// This returns the baud rate
    pub fn get_baud_rate(&self) -> u64 {
        self.baud_rate
    }
}

/// Loads and defines configurations related to dmx
impl DmxConfigStore {
    /// This creates, fills and returns a DmxConfigStore object
    pub fn new() -> DmxConfigStore {
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

        let dmx_fixtures_key = match dmx["fixtures"].as_array() {
            Some(e) => e,
            None => panic!("config does not contain key dmx/fixtures (array needed)")
        };

        let mut dmx_fixtures: Vec<String> = vec!();
        for e in dmx_fixtures_key.iter() {
            dmx_fixtures.push(e.as_str().expect("could not convert dmx/fixtures[i] as_str").to_string());
        }

        let animations_key = match dmx["animations"].as_array() {
            Some(e) => e,
            None => panic!("config does not contain key dmx/animations (array needed)")
        };

        let mut animations: Vec<String> = vec!();
        for e in animations_key.iter() {
            animations.push(e.as_str().expect("could not convert dmx/animations[i] as_str").to_string());
        }

        let quickanimations_key = match dmx["quickanimations"].as_array() {
            Some(e) => e,
            None => panic!("config does not contain key dmx/quickanimations (array needed)")
        };

        let mut quickanimations: Vec<String> = vec!();
        for e in quickanimations_key.iter() {
            quickanimations.push(e.as_str().expect("could not convert dmx/quickanimations[i] as_str").to_string());
        }

        DmxConfigStore {
            dmx_fixtures: dmx_fixtures,
            animations: animations,
            quickanimations: quickanimations
        }
    }
    /// Returns dmx_fixtures
    pub fn get_dmx_fixtures(&self) -> &Vec<String> {
        &self.dmx_fixtures
    }
    /// Returns animations
    pub fn get_animations(&self) -> &Vec<String> {
        &self.animations
    }
    /// Returns quickanimations
    pub fn get_quickanimations(&self) -> &Vec<String> {
        &self.quickanimations
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