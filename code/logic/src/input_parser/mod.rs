use crate::led_renderer::LedRenderer;
use crate::dmx_renderer::DmxRenderer;

use crate::config_store::InputConfigStore;

pub struct InputParser<'a> {
    button_states: Vec<u8>,
    input_config_store: &'a InputConfigStore
}

fn main() {
    impl<'a> InputParser<'a> {
        pub fn new(input_config_store: &InputConfigStore) -> InputParser {
            let mut actual_buttons = vec!();
            for i in 0..input_config_store.get_button_count() {
                actual_buttons.push(0);
            }
            InputParser {
                button_states: actual_buttons,
                input_config_store: input_config_store
            }
        }
        pub fn process_input(&self, led_renderer: &LedRenderer, dmx_renderer: &DmxRenderer, input_type: &str) -> bool {
            // do render stuff
            println!("Parsing Input for {} buttons", self.input_config_store.get_button_count());
            led_renderer.spawn_snake();
            dmx_renderer.all_up();

            if self.button_states.len() > 0 {
                false
            } else {
                true
            }
        }
    }
}