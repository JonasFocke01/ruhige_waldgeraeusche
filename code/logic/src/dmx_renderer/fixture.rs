use crate::config_store::DmxConfigStore;
use crate::logging;

use std::path::Path;

/// This should indicate the fixtures type
pub enum FixtureType {
    /// The fixture is some sort of a scanner
    Scanner
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
    animations: Vec<Vec<(u8, u8, bool, bool, bool, bool)>>,
    /// Stores the current animation the fixture is in
    current_animation: u8,
    /// The fixtures dimm value
    dimmer: u8,
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
            stage_coordinates: (0, 0),
            animations: animations,
            current_animation: 0,
            dimmer: 255,
            current_color: ((150.0, 0.0, 0.0), 60),
            light_mode_up: true,
            light_mode_down: true,
            light_mode_in: false,
            light_mode_out: false,
            enabled: true

            
        }
    }
    // Todo: this can possibly done in one function now
    // Todo: the parsing should only happen once to not read one file multiple times
    /// reads the animation files and returns the constructed animation vec
    fn read_animation_files(fixture_id: u8, dmx_config_store: &DmxConfigStore) -> Vec<Vec<(u8, u8, bool, bool, bool, bool)>> {
        let mut animations: Vec<Vec<(u8, u8, bool, bool, bool, bool)>> = vec!();
        let fixture_count = dmx_config_store.get_dmx_fixtures().len();
        for _ in dmx_config_store.get_dmx_fixtures().iter() {
            animations.push(vec!());
        }

        animations[0] = DmxFixture::help_read_animation_files(fixture_id, "snake.tpl", fixture_count as u8);

        animations
    }
    /// helper function for read_animation_files <br>
    /// returns the positions for each fixture from a given file for x fixtures
    fn help_read_animation_files(fixture_id: u8, animation_file_name: &str, fixture_count: u8) -> Vec<(u8, u8, bool, bool, bool, bool)> {
        let mut fixture_result: Vec<(u8, u8, bool, bool, bool, bool)> = vec!();
        let mut plain_content = match std::fs::read_to_string(Path::new((String::from("src/dmx_renderer/") + animation_file_name).as_str())) {
            Ok(e) => e,
            Err(e) => {
                logging::log(format!("Error occured while reading animation file {} {}", animation_file_name, e).as_str(), logging::LogLevel::Warning, true);
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

            // Todo: improve error handling
            let mut line_params = line.split(",");
            let x: u8 = line_params.next().unwrap().parse().expect(format!("Too few fixtures found in file {}. {} required", animation_file_name, fixture_count).as_str());
            let y: u8 = line_params.next().unwrap().parse().unwrap();
            let dir_up: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };
            let dir_down: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };
            let dir_in: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };
            let dir_out: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };
            
            fixture_result.push((x, y, dir_up, dir_down, dir_in, dir_out));
        }
        logging::log(format!("successfully parsed animation file {}", animation_file_name).as_str(), logging::LogLevel::Info, false);
        print!("{:?}\n", fixture_result);
        fixture_result
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
    /// Todo: the animations are stored as u8 therefore there must be a human readable doc of (animation == u8)
    pub fn set_current_animation(&mut self, animation: u8) {
        self.current_animation = animation;
    }
    /// returns the dimmer of the fixture
    pub fn get_dimmer(&self) -> u8 {
        self.dimmer
    }
    /// sets the dimmer of the fixture
    pub fn set_dimmer(&mut self, dimm: u8) {
        self.dimmer = dimm;
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
    // Todo: this should calculate the one byte representation
    pub fn set_current_color(&mut self, color: ((f32, f32, f32), u8)) {
        self.current_color = color;
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
        let position_index = position_index % self.animations[self.current_animation as usize].len() as u64;
        //print!("position_index: {}\n", position_index);
        match self.fixture_name.as_str() {
            "Victory Scan" => {
                footprint.push(self.animations[self.current_animation as usize][position_index as usize].0);
                footprint.push(self.animations[self.current_animation as usize][position_index as usize].1);
                footprint.push(8);
                footprint.push(0);
                footprint.push(self.current_color.1);
                //footprint.push(if self.animations[scanner_i as usize].2 { 60 /*Todo: Is this the dimmer? */ } else { 0 });// Todo: fixme

                footprint.push(100);
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
