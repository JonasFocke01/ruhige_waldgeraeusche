use crate::config_store::DmxConfigStore;

pub struct DmxRenderer<'a> {
    channels: [u8; 512],
    dmx_config_store: &'a DmxConfigStore
}

fn main() {
    impl<'a> DmxRenderer<'a> {
        pub fn new(dmx_config_store: &DmxConfigStore) -> DmxRenderer {
            DmxRenderer {
                channels: [0; 512],
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
}