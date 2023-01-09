use crate::config_store::DmxConfigStore;
use crate::logging;

use std::path::Path;

/// This should indicate the fixtures type
#[derive(PartialEq)]
pub enum FixtureType {
    /// The fixture is some sort of a scanner
    Scanner
}

/// Holds all possible animation types
#[derive(Clone, Copy)]
pub enum AnimationType {
    /// This is for all normal animations
    Animation,
    /// This is for all quickanimations
    Quickanimation,
    /// This is for all animations that are more off effect on current animations than animations itself
    Effect
}

/// All fixture must implement this to be handled by the dmx renderer
pub struct DmxFixture {
    /// The name of the fixture
    fixture_name: String,
    /// The type of the fixture
    fixture_type: FixtureType,
    /// The stage is divided into a 255 * 255 grid where each fixture has coordinates of its location on the stage
    stage_coordinates: (u8, u8),
    /// The animations of this fixture parsed from the tpl files <br>
    /// The inner vec is left to right when looking towards the stage <br>
    /// The up, down, in, out bools indicate, in which direction the fixture is moving <br>
    /// The Index vec contains the individual steps the fixture should take <br>
    /// Animation< Index< x_position, y_position, up, down, in, out >>
    animations: Vec<(String, Vec<(u8, u8, bool, bool, bool, bool, f32)>)>,
    /// Stores the current animation the fixture is in
    current_animation: String,
    /// The type of the current animation
    current_animation_type: AnimationType,
    /// The fixtures dimm value
    brightness: f32,
    /// The fixtures color as a tuple containing <br>
    /// the RGB representation at all times <br>
    /// the one-byte representation for that fixure if it is needed, otherwise always 0
    current_color: ((f32, f32, f32), u8),
    /// The fixture should light up if this is true and the current animations direction is up
    light_mode_up: bool,
    /// The fixture should light up if this is true and the current animations direction is down
    light_mode_down: bool,
    /// The fixture should light up if this is true and the current animations direction is in
    light_mode_in: bool,
    /// The fixture should light up if this is true and the current animations direction is out
    light_mode_out: bool,
    /// This indicates if the fixture is enabled and follows its animation or if it performs a blackout
    enabled: bool
}

/// This is responsible for holding all values a DmxFixture needs
/// This is created multiple times in the DmxRenderer
impl DmxFixture {
    /// The constructor fills all fields by taking in the fixture id, the fixture name and the dmx_config_store
    pub fn new(fixture_id: u8, fixture_name: String, dmx_config_store: &DmxConfigStore) -> DmxFixture {
        let animations = DmxFixture::read_animation_files(fixture_id, dmx_config_store);

        let fixture_type = match fixture_name.as_str() {
            "Victory Scan" => FixtureType::Scanner,
            e => {
                logging::log(format!("unknown fixture name found: {}", e).as_str(), logging::LogLevel::Warning, true);
                unreachable!();
            }
        };

        DmxFixture {
            fixture_name: fixture_name,
            fixture_type: fixture_type,
            stage_coordinates: (125, 0),
            animations: animations,
            current_animation: "test".to_string(),
            current_animation_type: AnimationType::Animation,
            brightness: 1.0,
            current_color: ((150.0, 0.0, 0.0), 60),
            light_mode_up: true,
            light_mode_down: true,
            light_mode_in: true,
            light_mode_out: true,
            enabled: true
        }
    }
    // Todo: (long term) the parsing should only happen once to not read one file multiple times
    /// reads the animation files and returns the constructed animation vec
    fn read_animation_files(fixture_id: u8, dmx_config_store: &DmxConfigStore) -> Vec<(String, Vec<(u8, u8, bool, bool, bool, bool, f32)>)> {
        let mut animations: Vec<(String, Vec<(u8, u8, bool, bool, bool, bool, f32)>)> = vec!();
        let fixture_count = dmx_config_store.get_dmx_fixtures().len();
        for _ in 0..dmx_config_store.get_animations().len() {
            animations.push(("placeholder".to_string(), vec!()));
        }

        for (animation_i, animation_name) in dmx_config_store.get_animations().iter().enumerate() {
            let mut plain_content = match std::fs::read_to_string(Path::new((String::from("src/dmx_renderer/") + animation_name + ".tpl").as_str())) {
                Ok(e) => e,
                Err(e) => {
                    logging::log(format!("Error occured while reading animation file {} {}", animation_name, e).as_str(), logging::LogLevel::Warning, true);
                    panic!("Error occured while reading animation file {}\n", e);
                }
            };
            plain_content = plain_content.replace(" ", "");
            let vec_content = plain_content.split("\n");
            let mut parsed_line_i: isize = 0;
            for (line_i, line) in vec_content.clone().enumerate() {
                if line_i == 0 || line_i % (fixture_count + 1) as usize == 0 { continue; }
                parsed_line_i += 1;
                if (parsed_line_i + 1) % fixture_count as isize != fixture_id as isize { continue; }

                let mut line_params = line.split(",");
                let x: u8 = line_params.next().unwrap().parse().expect(format!("Too few fixtures found in file {}. {} required", animation_name, fixture_count).as_str());
                let y: u8 = line_params.next().unwrap().parse().expect(format!("Too few fixtures found in file {}. {} required", animation_name, fixture_count).as_str());
                let dir_up: bool = if line_params.next().unwrap().parse::<u8>().expect(format!("Too few fixtures found in file {}. {} required", animation_name, fixture_count).as_str()) == 1 { true } else { false };
                let dir_down: bool = if line_params.next().unwrap().parse::<u8>().expect(format!("Too few fixtures found in file {}. {} required", animation_name, fixture_count).as_str()) == 1 { true } else { false };
                let dir_in: bool = if line_params.next().unwrap().parse::<u8>().expect(format!("Too few fixtures found in file {}. {} required", animation_name, fixture_count).as_str()) == 1 { true } else { false };
                let dir_out: bool = if line_params.next().unwrap().parse::<u8>().expect(format!("Too few fixtures found in file {}. {} required", animation_name, fixture_count).as_str()) == 1 { true } else { false };
                let mut brightness: f32 = line_params.next().unwrap().parse::<f32>().expect(format!("Too few fixtures found in file {}. {} required", animation_name, fixture_count).as_str());
                if brightness < 0.0 || brightness > 1.0 {
                    logging::log(format!("brightness need to be in range 0.0 to 1.0 but found {} in file {}\n", brightness, animation_name).as_str(), logging::LogLevel::Warning, false);
                    brightness = 0.0;
                }
                
                animations[animation_i].0 = animation_name.to_string();
                animations[animation_i].1.push((x, y, dir_up, dir_down, dir_in, dir_out, brightness));
            }
        }

        animations
    }
    /// returns the stage_coordinates
    pub fn get_stage_coordinates(&self) -> (u8, u8) {
        self.stage_coordinates
    }
    /// sets the stage_coordinates
    pub fn set_stage_coordinates(&mut self, coordinates: (u8, u8)) {
        self.stage_coordinates = coordinates;
    }
    /// sets the current animation
    /// a mapping of animation to u8 can be found by enumerating the animations string array in the config file starting with 0
    pub fn set_current_animation(&mut self, animation_type: AnimationType, animation_name: String) {
        self.current_animation_type = animation_type;
        self.current_animation = animation_name;
    }
    /// returns the brightness of the fixture
    pub fn get_brightness(&self) -> f32 {
        self.brightness
    }
    /// sets the brightness of the fixture
    pub fn set_brightness(&mut self, mut brightness: f32) {
        if brightness < 0.0 || brightness > 1.0 {
            logging::log(format!("brightness need to be in range 0.0 to 1.0 but attempt to set to {}\n", brightness).as_str(), logging::LogLevel::Warning, false);
            brightness = 0.0;
        }
        self.brightness = brightness;
    }
    /// returns the current color of the fixture
    pub fn get_current_color(&self) -> ((f32, f32, f32), u8) {
        self.current_color
    }
    /// returns the type of the fixture
    pub fn get_type(&self) -> &FixtureType {
        &self.fixture_type
    }
    /// sets the color of the fixture
    /// Maps the RGB value to its Dmx-one-channel-couterpart if None is given for the second value in p_color
    pub fn set_current_color(&mut self, p_color: ((f32, f32, f32), Option<u8>)) {
        
        if p_color.1.is_none() {
            let mut result_color: ((f32, f32, f32), u8) = (p_color.0, 0);
            match self.fixture_name.as_str() {
                "Victory Scan" =>   {
                                        if p_color.0.0 > p_color.0.1 + 150.0 && p_color.0.0 > p_color.0.2 + 150.0 {
                                            result_color.1 = 60 // red
                                        } else if p_color.0.1 > p_color.0.0 + 150.0 && p_color.0.1 > p_color.0.2 + 150.0 {
                                            result_color.1 = 49 // green
                                        } else if p_color.0.2 > p_color.0.0 + 150.0 && p_color.0.2 > p_color.0.1 + 150.0 {
                                            result_color.1 = 89 // blue
                                        } else if p_color.0.0 > p_color.0.1 && p_color.0.1 > p_color.0.2 {
                                            result_color.1 = 99 // orange
                                        } else if p_color.0.0 > p_color.0.2 + 200.0 && p_color.0.1 > p_color.0.2 + 200.0 {
                                            result_color.1 = 16 // yellow
                                        } else if p_color.0.0 < p_color.0.1 && p_color.0.1 < p_color.0.2 {
                                            result_color.1 = 34 // light blue
                                        } else if p_color.0.0 > p_color.0.2 && p_color.0.2 > p_color.0.1 {
                                            result_color.1 = 117 // pink
                                        }else if p_color.0.0 == p_color.0.2 && p_color.0.2 > p_color.0.1 {
                                            result_color.1 = 80 // purple
                                        }
                                    },                                                                
                e => {
                    logging::log(format!("unknown fixture name found while setting current color: {}", e).as_str(), logging::LogLevel::Warning, true);
                }
            }
            self.current_color = result_color;
        } else {
            match p_color.1 {
                Some(n) => self.current_color = (p_color.0, n),
                None => self.current_color = (p_color.0, 0)
            }
        }
    }
    /// Sets the current light mode for up <br>
    /// toggles if light_mode_up: None
    pub fn set_current_light_mode_up(&mut self, light_mode_up: Option<bool>) {
        match light_mode_up {
            Some(e) => self.light_mode_up = e,
            None => self.light_mode_up = !self.light_mode_up
        }
    }
    /// Sets the current light mode for down <br>
    /// toggles if light_mode_down: None
    pub fn set_current_light_mode_down(&mut self, light_mode_down: Option<bool>) {
        match light_mode_down {
            Some(e) => self.light_mode_down = e,
            None => self.light_mode_down = !self.light_mode_down
        }
    }
    /// Sets the current light mode for in <br>
    /// toggles if light_mode_in: None
    pub fn set_current_light_mode_in(&mut self, light_mode_in: Option<bool>) {
        match light_mode_in {
            Some(e) => self.light_mode_in = e,
            None => self.light_mode_in = !self.light_mode_in
        }
    }
    /// Sets the current light mode for out <br>
    ///  toggles if light_mode_out: None
    pub fn set_current_light_mode_out(&mut self, light_mode_out: Option<bool>) {
        match light_mode_out {
            Some(e) => self.light_mode_out = e,
            None => self.light_mode_out = !self.light_mode_out
        }
    }
    /// This creates all dmx values needed to drive the hardware fixture
    pub fn get_dmx_footprint(&self, position_index: u64) -> Vec<u8> {
        let mut footprint = vec!();
        let current_animation_index: u8 = match self.animations.iter().position(|animation| animation.0 == self.current_animation) {
            Some(n) => n as u8,
            None => {
                logging::log(format!("Animation {} not found in config file, setting current animation to default", self.current_animation).as_str(), logging::LogLevel::Warning, true);
                0
            }
        };
        let position_index = position_index % self.animations[current_animation_index as usize].1.len() as u64;
        match self.fixture_name.as_str() {
            "Victory Scan" => {
                footprint.push(self.animations[current_animation_index as usize].1[position_index as usize].0);
                footprint.push(self.animations[current_animation_index as usize].1[position_index as usize].1);
                footprint.push(8);
                footprint.push(0);
                footprint.push(self.current_color.1);
                if  (self.light_mode_up && self.animations[current_animation_index as usize].1[position_index as usize].2)   ||
                    (self.light_mode_down && self.animations[current_animation_index as usize].1[position_index as usize].3) ||
                    (self.light_mode_in && self.animations[current_animation_index as usize].1[position_index as usize].4)   ||
                    (self.light_mode_out && self.animations[current_animation_index as usize].1[position_index as usize].5)   {
                        footprint.push((255 as f32 * self.animations[current_animation_index as usize].1[position_index as usize].6 * self.brightness) as u8);
                } else {
                    footprint.push(0);
                }
                footprint.push(0);
            },
            e => logging::log(format!("unknown fixture name found: {}", e).as_str(), logging::LogLevel::Warning, true)
        }
        footprint
    }
    /// Returns if the fixture is enabled
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    /// Sets the fixtures enabled state
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }
}
