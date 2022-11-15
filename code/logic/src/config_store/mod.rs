use std::fs::File;

pub enum InputType {
    Serial,
    RestApi
}

pub struct InputConfigStore {
    button_count: u64,
    input_type: InputType
}

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

pub struct DmxConfigStore {
    channel_count: u64,
    scanner_count: u64
}

pub struct GeneralConfigStore {
    frame_timing: u64
}

impl LedConfigStore {
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
            parameter_count: leds["parameter_count"].as_u64().expect("config file does not contain expected sub key leds/parameter_count")
        }
    }
    pub fn get_led_count_per_strip(&self) -> u64 {
        self.led_count_per_strip
    }
    pub fn get_strip_count(&self) -> u64 {
        self.strip_count
    }
    pub fn get_led_pin(&self) -> u64 {
        self.led_pin
    }
    pub fn get_led_freq_hz(&self) -> u64 {
        self.led_freq_hz
    }
    pub fn get_led_dma(&self) -> u64 {
        self.led_dma
    }
    pub fn get_led_brightness(&self) -> u64 {
        self.led_brightness
    }
    pub fn get_led_invert(&self) -> u64 {
        self.led_invert
    }
    pub fn get_led_channel(&self) -> u64 {
        self.led_channel
    }
    pub fn get_pixel_offset(&self) -> u64 {
        self.pixel_offset
    }
    pub fn get_parameter_count(&self) -> u64 {
        self.parameter_count
    }
}

impl InputConfigStore {
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
    pub fn get_button_count(&self) -> u64 {
        self.button_count
    }
    pub fn get_input_type(&self) -> &InputType {
        &self.input_type
    }
}

impl DmxConfigStore {
    pub fn new() -> DmxConfigStore {
        let file = File::open("config.json")
                            .expect("config file should open read only");

        let json: serde_json::Value = serde_json::from_reader(file)
                    .expect("config file should be proper JSON");

        let input = json.get("dmx")
                    .expect("config file does not contain 'dmx' key");

        DmxConfigStore {
            channel_count: input["channel_count"].as_u64().expect("config file does not contain expected sub key dmx/channel_count"),
            scanner_count: input["scanner_count"].as_u64().expect("config file does not contain expected sub key dmx/scanner_count")
        }
    }
    pub fn get_channel_count(&self) -> u64 {
        self.channel_count
    }
    pub fn get_scanner_count(&self) -> u64 {
        self.scanner_count
    }
}

impl GeneralConfigStore {
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
    pub fn get_frame_timing(&self) -> u64 {
        self.frame_timing
    }
}

#[test]
fn test_general_config_store() {
    let general_config_store = GeneralConfigStore::new();
    assert!(general_config_store.get_frame_timing() > 0 as u64);
}

#[test]
fn test_dmx_config_store() {
    let dmx_config_store = DmxConfigStore::new();
    assert!(dmx_config_store.get_channel_count() > 0 as u64);
}

#[test]
fn test_led_config_store() {
    let led_config_store = LedConfigStore::new();
    assert!(led_config_store.get_parameter_count() > 0 as u64);
}

#[test]
fn test_input_config_store() {
    let input_config_store = InputConfigStore::new();
    assert!(input_config_store.get_button_count() > 0 as u64);
}

#[test]
fn test_input_config_store_input_type() {
    let input_config_store = InputConfigStore::new();
    match input_config_store.get_input_type() {
        InputType::Serial => (),
        InputType::RestApi => ()
    }
}