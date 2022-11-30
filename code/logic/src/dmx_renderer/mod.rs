use crate::scanner::Scanner;

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

/// Responsible for
/// - collecting the current state of
///     - scanners
///     - // todo: implement other fixtures
/// - processing them by building a dmx ready vec of 512 bytes
/// - writing the build vec to the usb connected dmx adapter
impl DmxRenderer {
    /// This creates, fills and returns the DmxRenderer object
    /// - opens the serial port to the dmx adapter
    pub fn new() -> DmxRenderer {
        let render_timestamp = Instant::now();

        let port = SerialPort::open("/dev/ttyUSB0", 115_200)
		    .map_err(|e| eprintln!("Error: Failed to open {}: {}", "/dev/ttyUSB0", e)).unwrap();

        DmxRenderer {
            render_timestamp: render_timestamp,
            dmx_port: port,
            updateable: false
        }
    }
    /// Gathers all usefull infomations of scanners etc., builds a 512 size vec and writes it to the dmx adapter
    /// only writes after 50 ms passed since last run AND an actual update can be written
    pub fn render(&mut self, scanner: &Scanner) -> Result<Vec<u8>, String> {

        if self.updateable && self.render_timestamp.elapsed().as_millis() >= 50 {
                        
            // ? dmx value array construction
            let mut channel_vec: Vec<u8> = vec!();
            
            // ? start byte
            channel_vec.push(69);

            // ? stage lights
            channel_vec.push(0);
            channel_vec.push(0);
            channel_vec.push(0);
            
            // ? scanner
            let scanner_positions = scanner.get_current_position();
            let scanner_color = scanner.get_current_color();
            for scanner_i in 0..scanner_positions.len() {
                channel_vec.push(scanner_positions[scanner_i as usize].0);
                channel_vec.push(scanner_positions[scanner_i as usize].1);
                channel_vec.push(8);
                channel_vec.push(0);
                channel_vec.push(*scanner_color);
                channel_vec.push(if scanner_positions[scanner_i as usize].2 { 60 } else { 0 });
                channel_vec.push(0);
            }
            
            // ? strobe
            //Todo: write strobe

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
    /// toggles if updateable: None
    pub fn set_updateable(&mut self, updateable: Option<bool>) -> bool {
        self.updateable = updateable.unwrap_or(true);
        self.updateable
    }
}

#[test]
fn dmx_rendering_works_as_expected() {
    use std::{thread, time};
    use crate::config_store::DmxConfigStore;

    let dmx_config_store = DmxConfigStore::new();
    let scanner = Scanner::new(&dmx_config_store);
    let mut dmx_renderer = DmxRenderer::new();
    dmx_renderer.set_updateable(None);
    thread::sleep(time::Duration::from_millis(100));
    assert_eq!(dmx_renderer.render(&scanner).unwrap().len(), 513);
    dmx_renderer.set_updateable(Some(false));
    thread::sleep(time::Duration::from_millis(100));
    assert_eq!(dmx_renderer.render(&scanner).unwrap().len(), 0);
}