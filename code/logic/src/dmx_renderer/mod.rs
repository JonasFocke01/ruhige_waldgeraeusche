use crate::config_store::{InputConfigStore, DmxConfigStore};
use crate::logging;

/// A separate module that contains the DmxFixture struct and implementation
/// The DmxFixture struct is responsible for a single DmxFixture and is created multiple times from the DmxRenderer
pub mod fixture;
use fixture::DmxFixture;
use fixture::FixtureType;

use std::time::Instant;
use serial2::SerialPort;

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
    updateable: bool
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
            updateable: false
        }
    }
    /// Gathers all dmx footprints from all available DmxFixtures
    /// Builds and writes the channel vector
    /// The Vector is prepended with a 69 as a startbyte
    /// If the Vector is less than 513 bytes, it will be appended with zeros
    pub fn render(&mut self) -> Result<Vec<u8>, String> {

        if self.position_timestamp.elapsed().as_millis() > 100 {
            self.position_index += 1;
            self.updateable = true;
            self.position_timestamp = Instant::now();
        }

        if self.render_timestamp.elapsed().as_millis() >= 50 {

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
                logging::log(format!("Occupied dmx-channels: {}", channel_vec.len() - 1).as_str(), logging::LogLevel::Info, true);
                self.print_dmx_channel_ocupied = false;
            }

            //logging::log(format!("channel_vec: {:?}", channel_vec).as_str(), logging::LogLevel::Info, true);

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
        for _ in fixture_types.iter() {
            for fixture in self.fixtures.iter_mut() {
                // Todo: this should truly react to the fixture type
                // if fixture_type == fixture.get_type() {
                    fixture.set_current_color(color);
                // }
            }
        }
    }
}