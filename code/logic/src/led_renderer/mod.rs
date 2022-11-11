/*
The led array has 6 values
0 -> red
1 -> green
2 -> blue
3 -> fading (brigtness is decreasing each run by X amount)
4 -> moving direction ( > 0 -> moving up, = 0 -> stationary, < 0 moving down )
*/

use std::process::{Command, Stdio, Child};

use crate::config_store::LedConfigStore;

pub struct LedRenderer<'a> {
    pixels: Vec<Vec<Vec<u8>>>,
    python_instance: Child,
    led_config_store: &'a LedConfigStore
}

impl<'a> LedRenderer<'a> {
    pub fn new(led_config_store: &LedConfigStore) -> LedRenderer {
        let mut actual_pixels = vec!();
        for i in 0..led_config_store.get_strip_count() {
            actual_pixels.push(vec!());
            for j in 0..led_config_store.get_led_count_per_strip() {
                actual_pixels[i as usize].push(vec!());
                for _ in 0..led_config_store.get_parameter_count() {
                    actual_pixels[i as usize][j as usize].push(0);
                }
            }
        }
        LedRenderer {
            pixels: actual_pixels,
            python_instance: Command::new("python3")
                                .arg("python/main.py")
                                .stdin(Stdio::piped())
                                .spawn()
                                .unwrap(),
            led_config_store: led_config_store
        }
    }
    pub fn spawn_snake(&self) {
        println!("spawning {} snake...", self.led_config_store.get_led_brightness());
    }
    pub fn render(&self) -> bool {
        // do render stuff
        println!("Rendering Leds...");
        false
    }
    pub fn get_pixels(&self) -> &Vec<Vec<Vec<u8>>>{
        &self.pixels
    }
}

#[test]
fn pixels_vec_size() {
    let led_config_store = LedConfigStore::new();
    let led_renderer = LedRenderer::new(&led_config_store);
    assert!(led_renderer.get_pixels().len() == led_config_store.get_strip_count() as usize);
    assert!(led_renderer.get_pixels()[0].len() == led_config_store.get_led_count_per_strip() as usize);
    assert!(led_renderer.get_pixels()[0][0].len() == led_config_store.get_parameter_count() as usize);
}