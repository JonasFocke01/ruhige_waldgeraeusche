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
}