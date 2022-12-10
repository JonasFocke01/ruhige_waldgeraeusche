use crate::config_store::LedConfigStore;
use crate::config_store::GlobalVarsStore;
use crate::logger;

use std::time::Instant;
use std::io::Write;
use std::process::{Command, Stdio, ChildStdin};
use rand::Rng;
use std::f64;

/// Holds every possible animation
pub enum Animation {
    /// A ~12 pixel long snake that runns from one strip end to the other with a speed of ~3, where every pixel is not as bright as the pixel before it
    Snake,
    /// A Block of ~12 pixels that flashes in, has no movement and fades out
    FadingBlocks,
    /// Flash the whole strip and fade every pixel
    FlashFadeWholeStrip,
    /// The brightness of every pixel is 0.0
    Blackout
}
/// All possible directions
pub enum Direction {
    /// Bottom to top
    Up,
    /// Top to bottom
    Down,
    /// Middle to edges
    Out
}
/// All possible combination of strips
pub enum Combination {
    /// All strips display the current animation simultaneusly
    Parallel,
    /// Every strip displays the current animation independend of each other
    Async
}

/// The struct to define how the LedRenderer should look like
pub struct LedRenderer<'a> {
    /// The actual pixels stored like Strip< Pixels< Parameters >>>
    pixels: Vec<Vec<Vec<f32>>>,
    /// The python script instance responsible for writing to the physical strip
    python_instance_stdin: ChildStdin,
    /// LedConfigStore contains usefull informations for the renderer
    led_config_store: &'a LedConfigStore,
    /// The last timestamp the leds where written. The leds framerate is capped to a hardcoded number of milliseconds
    render_timestamp: Instant,
    /// The animation displayed on the strips
    current_animation: Animation,
    /// The direction the animation goes
    current_direction: Direction,
    /// The combination on where to show the current animation
    current_combination: Combination
}

/// Responsible for storing and rendering the current pixel state 
impl<'a> LedRenderer<'a> {
    /// This creates, fills and returns the LedRenderer object
    /// - prefills the pixel array
    /// - spawns the python instance
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
                                    Err(error) => {
                                        logger::log(format!("{}", error).as_str());
                                        panic!("{}", error);
                                    }
                                };
        if python_instance.id() <= 0 {
            logger::log("Failed to spawn python instance!");
            panic!("Failed to spawn python instance");
        }
        let mut python_instance_stdin: ChildStdin = python_instance.stdin.unwrap();
        match python_instance_stdin.flush() {
            Ok(()) => (),
            Err(error) => {
                logger::log("Failed to flush python instance stdin");
                panic!("Failed to flush python instance stdin - {}", error);
            }
        };
        
        LedRenderer {
            pixels: actual_pixels,
            python_instance_stdin: python_instance_stdin,
            led_config_store: led_config_store,
            render_timestamp: render_timestamp,
            current_animation: Animation::Snake,
            current_direction: Direction::Up,
            current_combination: Combination::Parallel
        }
    }
    /// Spawns a snake <br>
    /// ! This is a test function !
    pub fn spawn_snake(&mut self, color: &(f32, f32, f32)) { 
        for strip_i in 0..self.led_config_store.get_strip_count() {   
            for index in self.led_config_store.get_pixel_offset()..12 {
                self.pixels[strip_i as usize][index as usize] = vec![color.0 * index as f32 / 12.0, color.1 * index as f32 / 12.0, color.2 * index as f32 / 12.0, 0.0, 3.0, 0.0];
            }
        }
    }
    /// Spawns a fading block  <br>
    /// ! This is a test function !
    pub fn spawn_fading_blocks(&mut self, color: &(f32, f32, f32)) {
        let mut rng = rand::thread_rng();
        for strip_i in 0..self.led_config_store.get_strip_count() { 
            let random_start = rng.gen_range(self.led_config_store.get_pixel_offset()..(self.led_config_store.get_led_count_per_strip() - 16));
            for index in random_start..(random_start + 15) {
                self.pixels[strip_i as usize][index as usize] = vec![color.0, color.1, color.2, 0.0, 0.0, 0.1];
            }
        }
    }
    /// Flashes the whole strip and lets it fade out <br>
    /// ! This is a test function !
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
    /// Renders the current state of the pixel vec to a format understandable by python and writes it to pythons stdin <br>
    /// Three bytes for each pixel (RGB)
    pub fn render(&mut self) -> Result<Vec<Vec<Vec<f32>>>, String> {
        let mut result_pixels: Vec<Vec<Vec<f32>>> = vec!();

        if self.render_timestamp.elapsed().as_millis() >= 5 {

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
            // ! Sometimes in the future, the priority of the pixels should be respected
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
            if  result_pixels.len() != self.led_config_store.get_strip_count() as usize &&
                result_pixels[0].len() != self.led_config_store.get_led_count_per_strip() as usize &&
                result_pixels[0][0].len() != self.led_config_store.get_parameter_count() as usize {
                    logger::log("result_pixels has the wrong size");
                    panic!("result_pixels has the wrong size");
            }
            self.pixels = result_pixels.to_vec();
            
            // ? draw
            let mut writable_pixels = vec!();
            for strip_i in 0..self.led_config_store.get_strip_count() {
                for mut pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                    if strip_i % 2 == 1 { pixel_i = GlobalVarsStore::map_range(pixel_i as f64, (0.0, self.led_config_store.get_led_count_per_strip() as f64 - 1.0), (self.led_config_store.get_led_count_per_strip() as f64 - 1.0, 0.0)) as u64; }
                    for parameter_i in 0..3 {
                        writable_pixels.push(self.pixels[strip_i as usize][pixel_i as usize][parameter_i as usize] as u8);
                    }
                }
            }
            for i in 0..writable_pixels.len() {
                match self.python_instance_stdin.write_all(format!("{} ", &writable_pixels[i]).as_bytes()) {
                    Ok(()) => (),
                    Err(_) => {
                        return Err(String::from("Error while writing to python stdin"));
                    }
                }
            }
            match self.python_instance_stdin.write_all("\n".as_bytes()) {
                Ok(()) => (),
                Err(_) => {
                    return Err(String::from("Error while writing to python stdin"));
                }
            }

            self.render_timestamp = Instant::now();
        }
        Ok(result_pixels)
    }
    /// Sets every pixel and its values to 0.0 to achieve a total blackout
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
    /// Triggers the function responsible for the current animation
    pub fn trigger_current_animation(&mut self, color: &(f32, f32, f32)) {
        match self.current_animation {
            Animation::Snake => self.spawn_snake(color),
            Animation::FadingBlocks => self.spawn_fading_blocks(color),
            Animation::FlashFadeWholeStrip => self.spawn_fading_blocks(color),
            Animation::Blackout => {
                self.clear_strips();
                return ()
            }
        }
    }
    /// Returns the current pixel vec
    pub fn get_pixels(&self) -> &Vec<Vec<Vec<f32>>> {
        &self.pixels
    }
    /// Returns the current animation
    pub fn get_current_animation(&self) -> &Animation {
        &self.current_animation
    }
    /// Sets the current animation
    pub fn set_current_animation(&mut self, animation: Animation) {
        self.current_animation = animation
    }
    /// Returns the current direction
    pub fn get_current_direction(&self) -> &Direction {
        &self.current_direction
    }
    /// Sets the current direction
    pub fn set_current_direction(&mut self, direction: Direction) {
        self.current_direction = direction
    }
    /// Returns the current combination
    pub fn get_current_combination(&self) -> &Combination {
        &self.current_combination
    }
    /// Sets the current combination
    pub fn set_current_combination(&mut self, combination: Combination) {
        self.current_combination = combination
    }
}