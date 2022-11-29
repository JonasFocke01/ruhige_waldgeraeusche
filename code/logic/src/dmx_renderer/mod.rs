use crate::config_store::DmxConfigStore;

use std::time::Instant;
use rand::Rng;
use serial2::SerialPort;

// Todo: this struct with its impl should be a file of its own
struct Scanner {
    // * all vecs are left to right when looking towards the stage
    // * Animation<Scanner<Position<x_position, y_position, direction_to_know_when_to_light_up_up, down, in, out
    animations: Vec<Vec<Vec<(u8, u8, bool, bool, bool, bool)>>>,
    active_animation: u8,
    current_color: u8,
    light_mode: Vec<ScannerLightMode>,
    lit_scanner: Vec<bool>
}

enum ScannerLightMode {
    In,
    Out,
    Up,
    Down,
    Blackout
}

impl Scanner {
    pub fn new(dmx_config_store:  &DmxConfigStore) -> Scanner {
        let animations = vec!();
        for _ in 0..1 {
            animations.push(vec!());
        }
        // Todo: develop custom file format for this to not clog up this file
        // testanimation1 = snake | only two scanner are implemented for this.
        animations[0].push(vec!());//push times scanner count
        animations[0].push(vec!());
        // position1
        animations[0][0][0].0 = 0;
        animations[0][0][0].1 = 0;
        animations[0][0][0].2 = true;
        animations[0][0][0].3 = false;
        animations[0][0][0].4 = false;
        animations[0][0][0].5 = true;

        animations[0][1][0].0 = 0;
        animations[0][1][0].1 = 0;
        animations[0][1][0].2 = true;
        animations[0][1][0].3 = false;
        animations[0][1][0].4 = false;
        animations[0][1][0].5 = true;

        // position2
        animations[0][0][1].0 = 255;
        animations[0][0][1].1 = 255;
        animations[0][0][1].2 = false;
        animations[0][0][1].3 = true;
        animations[0][0][1].4 = true;
        animations[0][0][1].5 = false;

        animations[0][1][1].0 = 255;
        animations[0][1][1].1 = 255;
        animations[0][1][1].2 = false;
        animations[0][1][1].3 = true;
        animations[0][1][1].4 = true;
        animations[0][1][1].5 = false;

        let mut lit_scanner = vec!();
        for _ in 0..2 {
            lit_scanner.push(true);
        }

        Scanner {
            animations: animations,
            active_animation: 0,
            current_color: 0,
            light_mode: ScannerLightMode::Blackout,
            lit_scanner:  lit_scanner
        }

    }
}

pub struct DmxRenderer<'a> {
    dmx_config_store: &'a DmxConfigStore,
    render_timestamp: Instant,
    dmx_port: SerialPort,
    updateable: bool
}

impl<'a> DmxRenderer<'a> {
    pub fn new(dmx_config_store: &DmxConfigStore) -> DmxRenderer {
        let render_timestamp = Instant::now();

        let port = SerialPort::open("/dev/ttyUSB0", 115_200)
		    .map_err(|e| eprintln!("Error: Failed to open {}: {}", "/dev/ttyUSB0", e)).unwrap();


        DmxRenderer {
            dmx_config_store: dmx_config_store,
            render_timestamp: render_timestamp,
            dmx_port: port,
            updateable: false
        }
    }
    pub fn render(&mut self) -> Result<Vec<u8>, String> {

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
            for scanner_i in 0..self.dmx_config_store.get_scanner_count() {
                channel_vec.push(self.scanner[scanner_i as usize][0]);
                channel_vec.push(self.scanner[scanner_i as usize][1]);
                channel_vec.push(0);
                channel_vec.push(0);
                channel_vec.push(self.scanner[scanner_i as usize][2]);
                channel_vec.push(255);
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
    pub fn set_updateable(&mut self, updateable: Option<bool>) -> bool {
        self.updateable = updateable.unwrap_or(!self.updateable);
        self.updateable
    }
}

#[test]
fn dmx_rendering_works_as_expected() {
    use std::{thread, time};

    let dmx_config_store = DmxConfigStore::new();
    let mut dmx_renderer = DmxRenderer::new(&dmx_config_store);
    dmx_renderer.set_updateable(None);
    thread::sleep(time::Duration::from_millis(100));
    assert_eq!(dmx_renderer.render().unwrap().len(), 513);
    dmx_renderer.set_updateable(Some(false));
    thread::sleep(time::Duration::from_millis(100));
    assert_eq!(dmx_renderer.render().unwrap().len(), 0);
}