use std::fs::File;

/// Enums all possible input types
pub enum InputType {
    /// This gathers input from serial
    Serial,
    /// This gathers input from a rest api
    /// Todo
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
    button_count: u64,
    input_type: InputType
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
    parameter_count: u64
}
/// The struct to define how the DmxConfigStore should look like
pub struct DmxConfigStore {
    scanner_count: u8
}
/// The struct to define how the GeneralConfigStore should look like
pub struct GeneralConfigStore {
    frame_timing: u64
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
        let file = File::open("config.json")
            .expect("config file should open read only");

        let json: serde_json::Value = serde_json::from_reader(file)
                    .expect("config file should be proper JSON");

        let leds = json.get("leds")
                    .expect("config file does not contain 'leds' key");

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
            // Todo: parameter count should be a const
            parameter_count: leds["parameter_count"].as_u64().expect("config file does not contain expected sub key leds/parameter_count")
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
}

/// Loads and defines configurations related to the input
impl InputConfigStore {
    /// This creates, fills and returns an InputConfigStore object
    pub fn new() -> InputConfigStore {
        let file = File::open("config.json")
            .expect("config file should open read only");

        let json: serde_json::Value = serde_json::from_reader(file)
                    .expect("config file should be proper JSON");

        let input = json.get("input")
                    .expect("config file does not contain 'input' key");

        let input_type = match input["input_type"].as_u64().expect("config file does not contain expected sub key input/input_type") {
                        1 => InputType::Serial,
                        2 => InputType::RestApi,
                        _ => panic!("Unexpected InputType in config.json!\nAvailable:\n    1: Serial\n    2:RestApi")
                    };
        
        InputConfigStore {
            button_count: input["button_count"].as_u64().expect("config file does not contain expected sub key input/button_count"),
            input_type: input_type
        }
    }
    /// Returns button_count
    pub fn get_button_count(&self) -> u64 {
        self.button_count
    }
    /// Returns input_type
    pub fn get_input_type(&self) -> &InputType {
        &self.input_type
    }
}

/// Loads and defines configurations related to dmx
impl DmxConfigStore {
    /// This creates, fills and returns a DmxConfigStore object
    pub fn new() -> DmxConfigStore {
        const SCANNER_COUNT: u8 = 5;

        DmxConfigStore {
            scanner_count: SCANNER_COUNT
        }
    }
    /// Returns scanner_count
    pub fn get_scanner_count(&self) -> u8 {
        self.scanner_count
    }
}

/// Loads and defines general configurations
impl GeneralConfigStore {
    /// This creates, fills and returns a GeneralConfigStore object
    pub fn new() -> GeneralConfigStore {
        let file = File::open("config.json")
            .expect("config file should open read only");

        let json: serde_json::Value = serde_json::from_reader(file)
                    .expect("config file should be proper JSON");

        let input = json.get("general")
                    .expect("config file does not contain 'general' key");

        GeneralConfigStore {
            frame_timing: input["frame_timing"].as_u64().expect("config file does not contain expected sub key general/frame_timing")
        }
    }
    /// Returns frame_timing
    pub fn get_frame_timing(&self) -> u64 {
        self.frame_timing
    }
}

/// Defines global variables
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
    /// Sets the color mode
    pub fn set_color_mode(&mut self, new_color_mode: ColorMode) {
        self.color_mode = new_color_mode;
    }
    /// Toggles the color_mode variable
    /// Todo: this can be integratet into set_color_mode
    pub fn toggle_color_mode(&mut self) {
        self.color_mode = match self.color_mode {
                                ColorMode::Primary => ColorMode::Complementary,
                                ColorMode::Complementary => ColorMode::Primary
                            };
    }
    /// Returns movement_mode
    pub fn get_movement_mode(&self) -> &MovementMode {
        &self.movement_mode
    }
    /// Sets the movement_mode varibale
    pub fn set_movement_mode(&mut self, new_movement_mode: MovementMode) {
        self.movement_mode = new_movement_mode;
    }
    /// Toggles the movement_mode variable
    /// Todo: this can be integratet into set_movement_mode
    pub fn toggle_movement_mode(&mut self) {
        self.movement_mode = match self.movement_mode {
                                MovementMode::Asynchronized => MovementMode::Synchronized,
                                MovementMode::Synchronized => MovementMode::Asynchronized
                            };
    }
}

#[test]
fn general_config_store_loaded_its_attributes_correctly() {
    let general_config_store = GeneralConfigStore::new();
    assert!(general_config_store.get_frame_timing() > 0 as u64);
}

#[test]
fn dmx_config_store_loaded_its_attributes_correctly() {
    let dmx_config_store = DmxConfigStore::new();
    assert!(dmx_config_store.get_scanner_count() > 0);
}

#[test]
fn led_config_store_loaded_its_attributes_correctly() {
    let led_config_store = LedConfigStore::new();
    assert!(led_config_store.get_parameter_count() > 0 as u64);
}

#[test]
fn input_config_store_loaded_its_attributes_correctly() {
    let input_config_store = InputConfigStore::new();
    assert!(input_config_store.get_button_count() > 0 as u64);
}

#[test]
fn input_config_store_input_type_loaded_and_parsed_its_attributes_correctly() {
    let input_config_store = InputConfigStore::new();
    match input_config_store.get_input_type() {
        InputType::Serial => (),
        InputType::RestApi => ()
    }
}