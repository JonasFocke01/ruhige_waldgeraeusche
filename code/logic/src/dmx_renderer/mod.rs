use crate::config_store::DmxConfigStore;

use std::time::Instant;

pub struct DmxRenderer<'a> {
    channels: Vec<u8>,
    dmx_config_store: &'a DmxConfigStore,
    render_timestamp: Instant
}

impl<'a> DmxRenderer<'a> {
    pub fn new(dmx_config_store: &DmxConfigStore) -> DmxRenderer {
        let mut actual_channels = vec!();
        for _ in 0..dmx_config_store.get_channel_count() {
            actual_channels.push(0);
        }
        let render_timestamp = Instant::now();

        DmxRenderer {
            channels: actual_channels,
            dmx_config_store: dmx_config_store,
            render_timestamp: render_timestamp
        }
    }
    pub fn all_up(&self) {
        // println!("all upping...");
    }
    pub fn render(&mut self) {
        // println!("Rendering Dmx {} times.", self.render_timestamp.elapsed().as_millis());
        self.render_timestamp = Instant::now();
    }
    pub fn get_channel_values(&self) -> &Vec<u8> {
        &self.channels
    }
}

#[test]
fn channel_size() {
    let dmx_config_store = DmxConfigStore::new();
    let dmx_renderer = DmxRenderer::new(&dmx_config_store);
    assert!(dmx_renderer.get_channel_values().len() == dmx_config_store.get_channel_count() as usize);
}