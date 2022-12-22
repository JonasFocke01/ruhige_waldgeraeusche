use crate::config_store::LedConfigStore;
use crate::config_store::GlobalVarsStore;
use crate::logging;

use std::time::Instant;
use std::io::Write;
use std::process::{Command, Stdio, ChildStdin};
use rand::Rng;

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

/// This struct contains all values associatet with an individual led
/// This is created (strip_count * pixels_per_strip) times
/// Todo: implement destination
/// Todo: each pixel should have own color
#[derive(Clone)]
pub struct Led {
    /// The brightness of each led in the range of 0.0 to 1.0
    /// Todo: enforce brightness range 0.0 to 1.0
    brightness: f32,
    /// The speed at which the pixel moves accross the strip
    /// Todo: test cases (-1.0, 0.0, 1.0)
    speed: f32,
    /// The brightness fading of each led
    /// > 1.0 -> fade in
    /// < 1.0 -> fade out
    /// = 1.0 -> neutral
    fade: f32
}

/// The struct to define how the LedRenderer should look like
pub struct LedRenderer<'a> {
    /// The actual pixels stored like Strip< Pixels< Parameters >>>
    pixels: Vec<Vec<Led>>,
    /// The python script instance responsible for writing to the physical strip
    python_instance_stdin: ChildStdin,
    /// LedConfigStore contains usefull informations for the renderer
    led_config_store: &'a LedConfigStore,
    /// The last timestamp the leds where written. The leds framerate is capped to a hardcoded number of milliseconds
    render_timestamp: Instant,
    // Todo: extract this variables into object to not clutter up LedRenderer
    /// The animation displayed on the strips
    current_animation: Animation,
    /// The direction the animation goes
    current_direction: Direction,
    /// The combination on where to show the current animation
    current_combination: Combination,
    // Todo: extract current color into object to not clutter up LedRenderer
    /// The current color of all led strips
    current_color: (f32, f32, f32),
    /// The next color of all led strips
    /// Todo: implement color transition
    dest_color: (f32, f32, f32),
    /// How much the color should transition each step <br>
    /// current_color: (100.0, 100.0, 100.0) <br>
    /// dest_color: (50.0, 50.0, 50.0) <br>
    /// color_transition_steps: 10.0 <br>
    /// current_color next iteration: (90.0, 90.0, 90.0)
    color_transition_steps: f32,
    /// if the rainbow mode is active (true), the color changes rapidly and unpredictable
    rainbow_mode: bool
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
            for _ in 0..led_config_store.get_led_count_per_strip() {
                actual_pixels[i as usize].push(Led {
                    brightness: 0.0,
                    speed: 0.0,
                    fade: 1.0
                });
            }
        }
        let render_timestamp = Instant::now();
        
        let python_instance = match Command::new("python3")
                                .arg("src/led_renderer/main.py")
                                .stdin(Stdio::piped())
                                .spawn() {
                                    Ok(n) => n,
                                    Err(error) => {
                                        logging::log(format!("{}", error).as_str(), logging::LogLevel::Warning, true);
                                        panic!("{}", error);
                                    }
                                };
        if python_instance.id() <= 0 {
            logging::log("Failed to spawn python instance!", logging::LogLevel::Warning, true);
            panic!("Failed to spawn python instance");
        }
        let mut python_instance_stdin: ChildStdin = python_instance.stdin.unwrap();
        match python_instance_stdin.flush() {
            Ok(()) => logging::log("successfully created python stdin", logging::LogLevel::Info, false),
            Err(error) => {
                logging::log("Failed to flush python instance stdin", logging::LogLevel::Warning, true);
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
            current_combination: Combination::Parallel,
            current_color: (255.0, 0.0, 0.0),
            dest_color: (100.0, 100.0, 100.0),
            color_transition_steps: 0.0,
            rainbow_mode: false
        }
    }
    /// Spawns a snake <br>
    /// ! This is a test function !
    pub fn spawn_snake(&mut self) { 
        for strip_i in 0..self.led_config_store.get_strip_count() {   
            for index in self.led_config_store.get_pixel_offset()..12 {
                self.pixels[strip_i as usize][index as usize] = Led {
                                                                brightness: index as f32 / 12.0,
                                                                speed: 3.0,
                                                                fade: 1.0
                                                            }
            }
        }
    }
    /// Spawns a fading block  <br>
    /// ! This is a test function !
    pub fn spawn_fading_blocks(&mut self) {
        let mut rng = rand::thread_rng();
        for strip_i in 0..self.led_config_store.get_strip_count() { 
            let random_start = rng.gen_range(self.led_config_store.get_pixel_offset()..(self.led_config_store.get_led_count_per_strip() - 16));
            for index in random_start..(random_start + 15) {
                self.pixels[strip_i as usize][index as usize] = Led {
                                                                brightness: 1.0,
                                                                speed: 0.0,
                                                                fade: 0.9
                                                            }
            }
        }
    }
    /// Flashes the whole strip and lets it fade out <br>
    /// ! This is a test function !
    pub fn flash_fade_whole_strip(&mut self) {
        for strip_i in 0..self.led_config_store.get_strip_count() {
            for pixel_i in self.led_config_store.get_pixel_offset()..self.led_config_store.get_led_count_per_strip() {
                if  self.pixels[strip_i as usize][pixel_i as usize].brightness < 10.0 {
                        self.pixels[strip_i as usize][pixel_i as usize] = Led {
                                                                        brightness: 1.0,
                                                                        speed: 0.0,
                                                                        fade: 1.0
                                                                    }
                }
            }
        }
    }
    /// Renders the current state of the pixel vec to a format understandable by python and writes it to pythons stdin <br>
    /// Three bytes for each pixel (RGB)
    pub fn render(&mut self) -> Result<Vec<Vec<Led>>, String> {
        let mut result_pixels: Vec<Vec<Led>> = vec!();

        // Todo: implement garbage collection on led strips (pixels who are brightness 0.0001 should not live long)
        if self.render_timestamp.elapsed().as_millis() >= self.led_config_store.get_frame_timing().into() {

            // ? rainbow mode

            if self.rainbow_mode {
                let new_color = (
                    if self.current_color.0 < 255.0 { self.current_color.0 + 1.0 } else { 0.0 },
                    if self.current_color.1 < 255.0 { self.current_color.1 + 5.0 } else { 0.0 },
                    if self.current_color.2 < 255.0 { self.current_color.2 + 2.0 } else { 0.0 },
                );
                self.current_color = new_color;
                self.dest_color = new_color;
            }

            // ? fade

            for strip_i in 0..self.led_config_store.get_strip_count() {
                for pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                    if  self.pixels[strip_i as usize][pixel_i as usize].brightness > 0.0 &&
                        self.pixels[strip_i as usize][pixel_i as usize].fade      != 0.0 {
                            self.pixels[strip_i as usize][pixel_i as usize].brightness *= self.pixels[strip_i as usize][pixel_i as usize].fade;
                    }
                }
            }

            // ? move

            for i in 0..self.led_config_store.get_strip_count() {
                result_pixels.push(vec!());
                for _ in 0..self.led_config_store.get_led_count_per_strip() {
                    result_pixels[i as usize].push(Led {
                        brightness: 0.0,
                        speed: 0.0,
                        fade: 1.0
                    });
                }
            }
            
            for strip_i in 0..self.led_config_store.get_strip_count() {
                for pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                    if  self.pixels[strip_i as usize][pixel_i as usize].brightness > 0.0 {
                        if  pixel_i as f32 + self.pixels[strip_i as usize][pixel_i as usize].brightness >= 0.0 &&
                            pixel_i as f32 + (self.pixels[strip_i as usize][pixel_i as usize].speed) < self.led_config_store.get_led_count_per_strip() as f32 - 1.0 {
                                result_pixels[strip_i as usize][(pixel_i as f32 + (self.pixels[strip_i as usize][pixel_i as usize].speed)) as usize] = self.pixels[strip_i as usize][pixel_i as usize].clone();
                        }
                    }
                }
            }
            
            self.pixels = result_pixels.to_vec();

            // ? draw
            
            let mut writable_pixels = vec!();
            for strip_i in 0..self.led_config_store.get_strip_count() {
                for mut pixel_i in 0..self.led_config_store.get_led_count_per_strip() {
                    if strip_i % 2 == 1 { pixel_i = GlobalVarsStore::map_range(pixel_i as f64, (0.0, self.led_config_store.get_led_count_per_strip() as f64 - 1.0), (self.led_config_store.get_led_count_per_strip() as f64 - 1.0, 0.0)) as u64; }
                    writable_pixels.push((self.pixels[strip_i as usize][pixel_i as usize].brightness as f32 * self.current_color.0) as u8);
                    writable_pixels.push((self.pixels[strip_i as usize][pixel_i as usize].brightness as f32 * self.current_color.1) as u8);
                    writable_pixels.push((self.pixels[strip_i as usize][pixel_i as usize].brightness as f32 * self.current_color.2) as u8);
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
        for _ in 0..self.led_config_store.get_strip_count() {
            actual_pixels.push(vec!());
            for _ in 0..self.led_config_store.get_led_count_per_strip() {
                Led {
                    brightness: 0.0,
                    speed: 0.0,
                    fade: 1.0
                };
            }
        }
        self.pixels = actual_pixels;
        true
    }
    /// Triggers the function responsible for the current animation
    pub fn trigger_current_animation(&mut self) {
        match self.current_animation {
            Animation::Snake => self.spawn_snake(),
            Animation::FadingBlocks => self.spawn_fading_blocks(),
            Animation::FlashFadeWholeStrip => self.spawn_fading_blocks(),
            Animation::Blackout => {
                self.clear_strips();
                return ()
            }
        }
    }
    /// Returns the current pixel vec
    pub fn get_pixels(&self) -> &Vec<Vec<Led>> {
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
    /// Sets the current color
    pub fn set_current_color(&mut self, color: (f32, f32, f32)) {
        self.current_color = color;
    }
    /// Sets the color, the current color should fade towards
    pub fn set_dest_color(&mut self, color: (f32, f32, f32)) {
        self.dest_color = color;
    }
    /// Sets the color transition steps
    pub fn set_color_transition_steps(&mut self, transition_steps: f32) {
        self.color_transition_steps = transition_steps;
    }
    /// Sets the rainbox mode
    pub fn set_rainbow_mode(&mut self, new_mode: bool) {
        self.rainbow_mode = new_mode;
    }
}