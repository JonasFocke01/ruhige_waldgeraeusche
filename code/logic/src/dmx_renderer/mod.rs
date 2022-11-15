use crate::config_store::DmxConfigStore;

use dmx::{self, DmxTransmitter};
use dmx_serial::posix::TTYPort;

use rand::Rng;

use std::time::Instant;

pub struct DmxRenderer<'a> {
    scanner: Vec<Vec<u8>>,
    dmx_config_store: &'a DmxConfigStore,
    render_timestamp: Instant,
    dmx_port: TTYPort
}

impl<'a> DmxRenderer<'a> {
    pub fn new(dmx_config_store: &DmxConfigStore) -> DmxRenderer {
        let mut scanner = vec!();
        for _ in 0..dmx_config_store.get_scanner_count() {
            // [x_position, y_position, color]
            scanner.push(vec![0, 0, 0]);
        }

        let render_timestamp = Instant::now();

        let dmx_port = match dmx::open_serial("/dev/ttyS0") {
            Ok(n) => n,
            Err(error) => panic!("Could not open dmx port: {}\n", error)
        };

        DmxRenderer {
            scanner: scanner,
            dmx_config_store: dmx_config_store,
            render_timestamp: render_timestamp,
            dmx_port: dmx_port
        }
    }
    pub fn all_up(&mut self) {
        // println!("all upping...");
        let mut rng = rand::thread_rng();
        for scanner_i in 0..self.dmx_config_store.get_scanner_count() {
            self.scanner[scanner_i as usize][0] = rng.gen_range(0..256) as u8;
            self.scanner[scanner_i as usize][1] = rng.gen_range(0..256) as u8;
            self.scanner[scanner_i as usize][2] = rng.gen_range(0..256) as u8;
            //print!("Random number: {}\n", rng.gen_range(0..256));
        }
    }
    pub fn render(&mut self) {

        if self.render_timestamp.elapsed().as_millis() >= 50 {
            print!("Rendering Dmx --------------------------------------------------");
            
            // ? move scanner
            //TODO
            
            // ? dmx value array construction
            let mut channel_vec: Vec<u8> = vec!();
            
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

            match self.dmx_port.send_dmx_packet(&channel_vec) {
                Ok(_) => self.render_timestamp = Instant::now(),
                Err(error) => print!("Error writing to dmx: {}\n", error)
            }
        }
    }
    pub fn get_scanner_values(&self) -> &Vec<Vec<u8>> {
        &self.scanner
    }
}

#[test]
fn channel_size() {
    let dmx_config_store = DmxConfigStore::new();
    let dmx_renderer = DmxRenderer::new(&dmx_config_store);
    assert!(dmx_renderer.get_scanner_values().len() == dmx_config_store.get_scanner_count() as usize);
}