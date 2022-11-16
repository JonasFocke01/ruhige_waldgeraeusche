use std::process::{Command, Stdio, ChildStdin};

use crate::config_store::LedConfigStore;

use std::time::Instant;

use std::io::Write;

use rand::Rng;

pub struct LedRenderer<'a> {
    pixels: Vec<Vec<Vec<f32>>>,
    python_instance_stdin: ChildStdin,
    led_config_store: &'a LedConfigStore,
    render_timestamp: Instant
}

impl<'a> LedRenderer<'a> {
    pub fn new(led_config_store: &LedConfigStore) -> LedRenderer {
        let mut actual_pixels = vec!();
        for i in 0..led_config_store.get_strip_count() {
            actual_pixels.push(vec!());
            for j in 0..led_config_store.get_led_count_per_strip() {
                actual_pixels[i as usize].push(vec!());
                for _ in 0..led_config_store.get_parameter_count() {
                    actual_pixels[i as usize][j as usize].push(0.0);
                }
            }
        }
        let render_timestamp = Instant::now();
        
        let python_instance = match Command::new("python3")
                                .arg("src/led_renderer/main.py")
                                .stdin(Stdio::piped())
                                .spawn() {
                                    Ok(n) => n,
                                    Err(error) => panic!("{}", error)
                                };
        assert!(python_instance.id() > 0, "Failed to spawn python instance!");
        let mut python_instance_stdin: ChildStdin = python_instance.stdin.unwrap();
        match python_instance_stdin.flush() {
            Ok(()) => (),
            Err(error) => panic!("Failed to flush python instance stdin - {}", error)
        };
        
        LedRenderer {
            pixels: actual_pixels,
            python_instance_stdin: python_instance_stdin,
            led_config_store: led_config_store,
            render_timestamp: render_timestamp
        }
    }
    pub fn spawn_snake(&mut self, color: &(f32, f32, f32)) {
        println!("spawning {} snake...", self.led_config_store.get_led_brightness());
        for strip_i in 0..self.led_config_store.get_strip_count() {   
            for index in self.led_config_store.get_pixel_offset()..12 {
                self.pixels[strip_i as usize][index as usize] = vec![color.0 * index as f32 / 12.0, color.1 * index as f32 / 12.0, color.2 * index as f32 / 12.0, 0.0, 3.0, 0.0];
            }
        }
    }
    pub fn spawn_fading_blocks(&mut self, color: &(f32, f32, f32)) {
        let mut rng = rand::thread_rng();
        for strip_i in 0..self.led_config_store.get_strip_count() { 
            // Todo: the start should not be random but represent the current pitch of some sort
            let random_start = rng.gen_range(self.led_config_store.get_pixel_offset()..(self.led_config_store.get_led_count_per_strip() - 16));
            for index in random_start..(random_start + 15) {
                self.pixels[strip_i as usize][index as usize] = vec![color.0, color.1, color.2, 0.0, 0.0, 0.1];
            }
        }
    }
    pub fn flash_fade_whole_strip(&mut self, color: &(f32, f32, f32)) {
        for strip_i in 0..self.led_config_store.get_strip_count() {
            for pixel_i in self.led_config_store.get_pixel_offset()..self.led_config_store.get_led_count_per_strip() {
                if  self.pixels[strip_i as usize][pixel_i as usize][0] < 10.0 || 
                    self.pixels[strip_i as usize][pixel_i as usize][1] < 10.0 || 
                    self.pixels[strip_i as usize][pixel_i as usize][2] < 10.0 {
                        self.pixels[strip_i as usize][pixel_i as usize] = vec![color.0, color.1, color.2, 0.0, 0.0, 0.1];
                    }
            }
        }
    }
    pub fn render(&mut self) -> Vec<Vec<Vec<f32>>> {
        // println!("Leds not rendered for {} ms.", self.render_timestamp.elapsed().as_millis());

        let mut result_pixels: Vec<Vec<Vec<f32>>> = vec!();

        // ? fade

        for strip_i in 0..self.led_config_store.get_strip_count() {
            for pixel_i in 0..self.led_config_store.get_led_count_per_strip() { 
                if  self.pixels[strip_i as usize][pixel_i as usize][0] > 0.0 || 
                    self.pixels[strip_i as usize][pixel_i as usize][1] > 0.0 || 
                    self.pixels[strip_i as usize][pixel_i as usize][2] > 0.0 {
                        self.pixels[strip_i as usize][pixel_i as usize][0] = (self.pixels[strip_i as usize][pixel_i as usize][0] * (1.0 - self.pixels[strip_i as usize][pixel_i as usize][5])) as f32;
                        self.pixels[strip_i as usize][pixel_i as usize][1] = (self.pixels[strip_i as usize][pixel_i as usize][1] * (1.0 - self.pixels[strip_i as usize][pixel_i as usize][5])) as f32;
                        self.pixels[strip_i as usize][pixel_i as usize][2] = (self.pixels[strip_i as usize][pixel_i as usize][2] * (1.0 - self.pixels[strip_i as usize][pixel_i as usize][5])) as f32;
                    }
            }
        }
        
        // ? move

        //Todo: the priority should be respected

        for i in 0..self.led_config_store.get_strip_count() {
            result_pixels.push(vec!());
            for j in 0..self.led_config_store.get_led_count_per_strip() {
                result_pixels[i as usize].push(vec!());
                for _ in 0..self.led_config_store.get_parameter_count() {
                    result_pixels[i as usize][j as usize].push(0.0);
                }
            }
        }
        
        for strip_i in 0..self.led_config_store.get_strip_count() {
            for pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                if  self.pixels[strip_i as usize][pixel_i as usize][0] > 0.0 || 
                    self.pixels[strip_i as usize][pixel_i as usize][1] > 0.0 || 
                    self.pixels[strip_i as usize][pixel_i as usize][2] > 0.0 {
                        if  pixel_i as f64 + self.pixels[strip_i as usize][pixel_i as usize][4] as f64 >= 0.0 &&
                            pixel_i as f64 + (self.pixels[strip_i as usize][pixel_i as usize][4] as f64) < self.led_config_store.get_led_count_per_strip() as f64 {
                                result_pixels[strip_i as usize][(pixel_i as f64 + (self.pixels[strip_i as usize][pixel_i as usize][4] as f64)) as usize] = self.pixels[strip_i as usize][pixel_i as usize].to_vec();
                        }
                }
            }
        }
        assert!(result_pixels.len() == self.led_config_store.get_strip_count() as usize);
        assert!(result_pixels[0].len() == self.led_config_store.get_led_count_per_strip() as usize);
        assert!(result_pixels[0][0].len() == self.led_config_store.get_parameter_count() as usize);
        self.pixels = result_pixels.to_vec();
        
        // ? draw

        // Todo: drawing should invert for odd number of strips 
        let mut writable_pixels = vec!();
        for strip_i in 0..self.led_config_store.get_strip_count() {
            for pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                for parameter_i in 0..3 {
                    let parameter = self.pixels[strip_i as usize][pixel_i as usize][parameter_i as usize];
                    if parameter == 10.0 {
                        writable_pixels.push(11);
                    } else {
                        writable_pixels.push(self.pixels[strip_i as usize][pixel_i as usize][parameter_i as usize] as u8);
                    }
                }
            }
        }
    
        for i in 0..writable_pixels.len() {
            match self.python_instance_stdin.write_all(format!("{} ", &writable_pixels[i]).as_bytes()) {
                Ok(()) => (),
                Err(error) => {
                    println!("Error while writing to python stdin: {}", error);
                    return vec!();
                }
            }
        }
        match self.python_instance_stdin.write_all("\n".as_bytes()) {
            Ok(()) => (),
            Err(error) => {
                println!("Error while writing to python stdin: {}", error);
                return vec!();
            }
        }

        self.render_timestamp = Instant::now();

        result_pixels
    }
    pub fn clear_strips(&mut self) -> bool {
        let mut actual_pixels = vec!();
        for i in 0..self.led_config_store.get_strip_count() {
            actual_pixels.push(vec!());
            for j in 0..self.led_config_store.get_led_count_per_strip() {
                actual_pixels[i as usize].push(vec!());
                for _ in 0..self.led_config_store.get_parameter_count() {
                    actual_pixels[i as usize][j as usize].push(0.0);
                }
            }
        }
        self.pixels = actual_pixels;
        true
    }
    pub fn get_pixels(&self) -> &Vec<Vec<Vec<f32>>>{
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