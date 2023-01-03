use crate::config_store::{InputConfigStore, DmxConfigStore, GlobalVarsStore};
use crate::logging;

/// A separate module that contains the DmxFixture struct and implementation
/// The DmxFixture struct is responsible for a single DmxFixture and is created multiple times from the DmxRenderer
pub mod fixture;
use fixture::DmxFixture;
use fixture::FixtureType;

use std::time::Instant;
use std::ops::Not;
use serial2::SerialPort;

/// The mode that determines how the fixtures should change theyr color
#[derive(Clone, Copy, PartialEq)]
pub enum ColorTransitionMode {
    /// The fixtures should change instant
    Instant,
    /// The fixtures should change theyr color animative
    Animative
}

impl Not for ColorTransitionMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ColorTransitionMode::Instant => ColorTransitionMode::Animative,
            ColorTransitionMode::Animative => ColorTransitionMode::Instant
        }
    }
}
/// The struct to define how a DmxRenderer should look like (more than one is possible and intended)
pub struct DmxRenderer {
    /// the fixture positions advance every 100 ms
    position_timestamp: Instant,
    /// limit writing to dmx once every 50ms to not confuse the adapter
    render_timestamp: Instant,
    /// The SerialPort object, the adapter is connected to
    dmx_port: SerialPort,
    /// This is the position index all fixtures need to calculate theyr currrent position
    position_index: u64,
    /// This stores all fixtures
    fixtures: Vec<DmxFixture>,
    /// Should only be true at the start of the programm to review how much dmx channels are used
    print_dmx_channel_ocupied: bool,
    /// Stores if an dmx update happened to further slow down dmx writing
    updateable: bool,
    color_transition_mode: ColorTransitionMode,
    color_transition_index: u8,
    color_transition_to_color: ((f32, f32, f32), u8),
    color_transition_speed: u128
}

/// Responsible for
/// - collecting the current state of all fixtures
/// - processing them by building a dmx ready vec of 512 bytes
/// - writing the build vec to the usb connected dmx adapter
impl DmxRenderer {
    /// This creates, fills and returns the DmxRenderer object
    /// - opens the serial port to the dmx adapter
    pub fn new(input_config_store: &InputConfigStore, dmx_config_store: &DmxConfigStore, serial_input_port: &str) -> DmxRenderer {
        let port = match SerialPort::open(format!("/dev/{}", serial_input_port), input_config_store.get_baud_rate() as u32) {
            Ok(e) => {
                logging::log("Successfully opened dmx-usb-adapter", logging::LogLevel::Info, false);
                e
            },
            Err(_) => {
                logging::log("Failed to open dmx-usb-adapter", logging::LogLevel::Warning, true);
                panic!("Failed to open dmx-usb-adapter");
            }
        };

        let mut fixtures: Vec<DmxFixture> = vec!();

        for (fixture_i, fixture) in dmx_config_store.get_dmx_fixtures().iter().enumerate() {
            fixtures.push(DmxFixture::new(fixture_i as u8, fixture.to_string(), &dmx_config_store));
        }

        DmxRenderer {
            position_timestamp: Instant::now(),
            render_timestamp: Instant::now(),
            dmx_port: port,
            position_index: 0,
            fixtures: fixtures,
            print_dmx_channel_ocupied: true,
            updateable: false,
            color_transition_mode: ColorTransitionMode::Instant,
            color_transition_index: 0,
            color_transition_to_color: ((0.0, 0.0, 0.0), 0),
            color_transition_speed: 25
        }
    }
    /// Gathers all dmx footprints from all available DmxFixtures
    /// Builds and writes the channel vector
    /// The Vector is prepended with a 69 as a startbyte
    /// If the Vector is less than 513 bytes, it will be appended with zeros
    pub fn render(&mut self) -> Result<Vec<u8>, String> {

        if self.color_transition_index < 255 && self.render_timestamp.elapsed().as_millis() % if self.color_transition_speed == 0 { 1 } else { self.color_transition_speed } == 0 {
            self.set_color(vec!(FixtureType::Scanner), self.color_transition_to_color);
            self.color_transition_index = self.color_transition_index + 1;
            logging::log(format!("color_transition_index at: {}", self.color_transition_index).as_str(), logging::LogLevel::Info, false);
        }

        if self.position_timestamp.elapsed().as_millis() > 100 {
            self.position_index += 1;
            self.updateable = true;
            self.position_timestamp = Instant::now();
        }

        if self.updateable && self.render_timestamp.elapsed().as_millis() >= 50 {

            // ? dmx value array construction
            let mut channel_vec: Vec<u8> = vec!();
            
            // ? start byte
            channel_vec.push(69);

            for fixture in self.fixtures.iter() {
                for parameter in fixture.get_dmx_footprint(self.position_index).iter() {
                    channel_vec.push(*parameter);
                }
            }
            
            if self.print_dmx_channel_ocupied {
                // Todo: log fixture count
                logging::log(format!("Occupied dmx-channels: {}", channel_vec.len()).as_str(), logging::LogLevel::Info, true);
                self.print_dmx_channel_ocupied = false;
            }

            while channel_vec.len() < 513 {
                channel_vec.push(0);
            }

            match self.dmx_port.write(&channel_vec) {
                Ok(_) => (),
                Err(_) => return Err("Error while writing to Serial DMX port".to_string())
            };

            self.updateable = false;
            self.render_timestamp = Instant::now();
            return Ok(channel_vec)
        }
        Ok(vec!())
    }
    /// This maps the given color to all fixtures given in the fixture_types parameter
    pub fn set_color(&mut self, fixture_types: Vec<FixtureType>, color: ((f32, f32, f32), u8)) {
        self.color_transition_to_color = color;
        if self.color_transition_mode == ColorTransitionMode::Animative && self.color_transition_index == 255 {
            self.color_transition_index = 0;
        }
        for fixture_type in fixture_types.iter() {
            for fixture in self.fixtures.iter_mut() {
                if fixture.get_stage_coordinates().0 < self.color_transition_index {
                    if fixture_type == fixture.get_type() {
                        fixture.set_current_color(color)
                    }
                }
                    
            }
        }
    }
    /// This sets the color transition mode
    /// Toggles the transition mode if None is given
    pub fn set_color_transition_mode(&mut self, new_color_transition_mode: Option<ColorTransitionMode>) {
        logging::log(format!("color transition mode changed!").as_str(), logging::LogLevel::Info, false);
        match new_color_transition_mode {
            Some(e) => self.color_transition_mode = e,
            None => self.color_transition_mode = !self.color_transition_mode
        }
    }
    /// This sets the color transition speed.
    /// This function maps the given u8 to 0 - 50
    pub fn set_color_transition_speed(&mut self, speed: u8) {
        self.color_transition_speed = GlobalVarsStore::map_range(speed.into(), (0.0, 255.0), (0.0, 50.0)) as u128;
    }
}