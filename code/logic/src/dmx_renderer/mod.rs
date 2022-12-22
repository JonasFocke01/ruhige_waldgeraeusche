use crate::config_store::InputConfigStore;
use crate::scanners::Scanners;
use crate::logging;

use std::time::Instant;
use serial2::SerialPort;

/// The struct to define how a DmxRenderer should look like (more than one is possible and intended)
pub struct DmxRenderer {
    /// limit writing to once every 50ms to not confuse the adapter
    render_timestamp: Instant,
    /// writing speed is further limited by this variable, which should indicate if an actual update happened, to further reduce confusion to the adapter
    updateable: bool,
    /// The SerialPort object, the adapter is connected to
    dmx_port: SerialPort
}
/// All the fixture must implement this to be handled by the dmx renderer
pub trait DmxFixture {
    /// The stage is divided into a 255 * 255 grid where each fixture has a coordinate
    fn get_coordinates(&self) -> (u8, u8);
    /// Should set the color of the fixture
    fn set_color(&mut self);
    /// Should set the brightness of the fixture
    fn set_brightness(&mut self);
}

/// Responsible for
/// - collecting the current state of
///     - scanners
/// - processing them by building a dmx ready vec of 512 bytes
/// - writing the build vec to the usb connected dmx adapter
impl DmxRenderer {
    /// This creates, fills and returns the DmxRenderer object
    /// - opens the serial port to the dmx adapter
    pub fn new(input_config_store: &InputConfigStore, serial_input_port: &str) -> DmxRenderer {
        let render_timestamp = Instant::now();

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

        DmxRenderer {
            render_timestamp: render_timestamp,
            dmx_port: port,
            updateable: false
        }
    }
    /// Gathers all usefull infomations of scanners etc., builds a 513 size vec and writes it to the dmx adapter
    /// only writes after 50 ms passed since last run AND an actual update can be written
    /// The current DMX channel configuration:
    /// channel_start-channel_end(channel_footprint): fixture_type - fixture_name
    ///   1-  7(  7):   scanner - JB Systems LED Victory Scan
    ///   8- 14(  7):   scanner - JB Systems LED Victory Scan
    ///  15- 21(  7):   scanner - JB Systems LED Victory Scan
    ///  22- 28(  7):   scanner - JB Systems LED Victory Scan
    ///  29- 35(  7):   scanner - JB Systems LED Victory Scan
    pub fn render(&mut self, scanners: &Scanners) -> Result<Vec<u8>, String> {

        if self.updateable && self.render_timestamp.elapsed().as_millis() >= 50 {
                        
            // ? dmx value array construction
            let mut channel_vec: Vec<u8> = vec!();
            
            // ? start byte
            channel_vec.push(69);
            
            // ? scanner
            let scanner_positions = scanners.get_current_position();
            let scanner_color = scanners.get_current_color();
            for scanner_i in 0..scanner_positions.len() {
                channel_vec.push(scanner_positions[scanner_i as usize].0);
                channel_vec.push(scanner_positions[scanner_i as usize].1);
                channel_vec.push(8);
                channel_vec.push(0);
                channel_vec.push(*scanner_color);
                channel_vec.push(if scanner_positions[scanner_i as usize].2 { 60 } else { 0 });
                channel_vec.push(0);
            }

            while channel_vec.len() < 513 {
                channel_vec.push(0);
            }

            match self.dmx_port.write(&channel_vec) {
                Ok(_) => (),
                Err(_) => return Err("Error while writing to Serial DMX port".to_string())
            };

            self.render_timestamp = Instant::now();
            self.updateable = false;
            return Ok(channel_vec)
        }
        Ok(vec!())
    }
    /// sets the updateable field <br>
    /// toggles if updateable = None
    pub fn set_updateable(&mut self, updateable: Option<bool>) {
        self.updateable = updateable.unwrap_or(true);
    }
}