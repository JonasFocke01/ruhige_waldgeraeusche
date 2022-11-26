use crate::config_store::DmxConfigStore;

use serial2::SerialPort;

use rand::Rng;

use std::time::Instant;


pub struct DmxRenderer<'a> {
    scanner: Vec<Vec<u8>>,
    dmx_config_store: &'a DmxConfigStore,
    render_timestamp: Instant,
    dmx_port: SerialPort
}

impl<'a> DmxRenderer<'a> {
    pub fn new(dmx_config_store: &DmxConfigStore) -> DmxRenderer {
        let mut scanner = vec!();
        for _ in 0..dmx_config_store.get_scanner_count() {
            scanner.push(vec![0, 0, 0]);
        }

        let render_timestamp = Instant::now();

        let port = SerialPort::open("/dev/ttyUSB0", 115_200)
		    .map_err(|e| eprintln!("Error: Failed to open {}: {}", "/dev/ttyUSB0", e)).unwrap();

        DmxRenderer {
            scanner: scanner,
            dmx_config_store: dmx_config_store,
            render_timestamp: render_timestamp,
            dmx_port: port
        }
    }
    pub fn all_up(&mut self) {
        let mut rng = rand::thread_rng();
        for scanner_i in 0..self.dmx_config_store.get_scanner_count() {
            self.scanner[scanner_i as usize][0] = rng.gen_range(0..256) as u8;
            self.scanner[scanner_i as usize][1] = rng.gen_range(0..256) as u8;
            self.scanner[scanner_i as usize][2] = rng.gen_range(0..256) as u8;
        }
    }
    pub fn render(&mut self) {

        if self.render_timestamp.elapsed().as_millis() >= 50 {
            
            // ? move scanner
            //TODO
            
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
            //TODO

            while channel_vec.len() < 514 {
                channel_vec.push(0);
            }

            //TODO: improve error handling
            match self.dmx_port.write(&channel_vec) {
                Ok(_) => (),
                Err(_) => print!("Error while writing to DMX")
            };

            self.render_timestamp = Instant::now();
        }
    }
    pub fn get_scanner_values(&self) -> &Vec<Vec<u8>> {
        &self.scanner
    }
}

#[test]
fn scanner_vec_has_expected_size() {
    let dmx_config_store = DmxConfigStore::new();
    let dmx_renderer = DmxRenderer::new(&dmx_config_store);
    assert!(dmx_renderer.get_scanner_values().len() == dmx_config_store.get_scanner_count() as usize);
}