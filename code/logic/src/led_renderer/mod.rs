use std::process::{Command, Stdio, ChildStdin};

use crate::config_store::LedConfigStore;

use std::time::Instant;

use std::io::Write;

// use std::{thread, time};

pub struct LedRenderer<'a> {
    pixels: Vec<Vec<Vec<u8>>>,
    python_instance_stdin: ChildStdin,
    led_config_store: &'a LedConfigStore,
    render_timestamp: Instant,
    test: bool
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
        let render_timestamp = Instant::now();
        
        let python_instance = match Command::new("python3")
                                .arg("src/led_renderer/main.py")
                                .stdin(Stdio::piped())
                                .spawn() {
                                    Ok(n) => n,
                                    Err(error) => panic!("{}", error)
                                };
        assert!(python_instance.id() > 0);
        let mut python_instance_stdin: ChildStdin = python_instance.stdin.unwrap();
        match python_instance_stdin.flush() {
            Ok(()) => (),
            Err(error) => panic!("{}", error)
        };
        let test = false;
        LedRenderer {
            pixels: actual_pixels,
            python_instance_stdin: python_instance_stdin,
            led_config_store: led_config_store,
            render_timestamp: render_timestamp,
            test: test
        }
    }
    pub fn spawn_snake(&mut self) {
        println!("spawning {} snake...", self.led_config_store.get_led_brightness());
        if self.test {
            self.pixels[0][1] = vec![150, 11, 11, 0, 0, 1];
            self.test = false;
        } else {
            self.pixels[0][1] = vec![0, 11, 11, 0, 0, 1];
            self.test = true;
        }
    }
    pub fn render(&mut self) -> Vec<Vec<Vec<u8>>> {
        // println!("Rendering Leds {} times.", self.render_timestamp.elapsed().as_millis());
        println!("Rendering Leds 1 time.");

        let mut result_pixels: Vec<Vec<Vec<u8>>> = vec!();
        
        // for _ in 0..self.render_timestamp.elapsed().as_millis() {

            // fade
            // for strip_i in 0..self.led_config_store.get_strip_count() {
            //     for pixel_i in 0..self.led_config_store.get_led_count_per_strip() { 
            //         self.pixels[strip_i as usize][pixel_i as usize][0] = self.pixels[strip_i as usize][pixel_i as usize][0] * self.pixels[strip_i as usize][pixel_i as usize][4];
            //         self.pixels[strip_i as usize][pixel_i as usize][1] = self.pixels[strip_i as usize][pixel_i as usize][1] * self.pixels[strip_i as usize][pixel_i as usize][4];
            //         self.pixels[strip_i as usize][pixel_i as usize][2] = self.pixels[strip_i as usize][pixel_i as usize][2] * self.pixels[strip_i as usize][pixel_i as usize][4];
            //     }
            // }
            
            // move
            for i in 0..self.led_config_store.get_strip_count() {
                result_pixels.push(vec!());
                for j in 0..self.led_config_store.get_led_count_per_strip() {
                    result_pixels[i as usize].push(vec!());
                    for _ in 0..self.led_config_store.get_parameter_count() {
                        result_pixels[i as usize][j as usize].push(0);
                    }
                }
            }
            assert!(result_pixels.len() == self.led_config_store.get_strip_count() as usize);
            assert!(result_pixels[0].len() == self.led_config_store.get_led_count_per_strip() as usize);
            assert!(result_pixels[0][0].len() == self.led_config_store.get_parameter_count() as usize);

            for strip_i in 0..self.led_config_store.get_strip_count() {
                for pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                    //Todo: respect boundries of strips
                    if self.pixels[strip_i as usize][pixel_i as usize][0] > 0 || self.pixels[strip_i as usize][pixel_i as usize][1] > 0 || self.pixels[strip_i as usize][pixel_i as usize][2] > 0 {
                        result_pixels[strip_i as usize][pixel_i as usize] = self.pixels[strip_i as usize][pixel_i as usize].to_vec();
                    }
                }
            }
        // }
        self.pixels = result_pixels.to_vec();
        
        // draw
        let mut writable_pixels = vec!();
        for strip_i in 0..self.led_config_store.get_strip_count() {
            for pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                for parameter_i in 0..3 {
                    let parameter = self.pixels[strip_i as usize][pixel_i as usize][parameter_i as usize];
                    if parameter == 10 {
                        writable_pixels.push(11);
                    } else {
                        writable_pixels.push(self.pixels[strip_i as usize][pixel_i as usize][parameter_i as usize]);
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
        // thread::sleep( time::Duration::from_millis(200) );

        result_pixels
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