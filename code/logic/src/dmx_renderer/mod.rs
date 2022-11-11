use crate::config_store::DmxConfigStore;

pub struct DmxRenderer<'a> {
    channels: Vec<u8>,
    dmx_config_store: &'a DmxConfigStore
}

impl<'a> DmxRenderer<'a> {
    pub fn new(dmx_config_store: &DmxConfigStore) -> DmxRenderer {
        let mut actual_channels = vec!();
        for _ in 0..dmx_config_store.get_channel_count() {
            actual_channels.push(0);
        }
        DmxRenderer {
            channels: actual_channels,
            dmx_config_store: dmx_config_store
        }
    }
    pub fn all_up(&self) {
        println!("all upping...");
    }
    pub fn render(&self) -> bool {
        // do render stuff
        println!("Rendering Dmx for {} channel...", self.dmx_config_store.get_channel_count());
        false
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