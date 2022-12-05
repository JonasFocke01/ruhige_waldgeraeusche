use crate::dmx_renderer::DmxRenderer;
use crate::config_store::DmxConfigStore;
use crate::logger;

use std::path::Path;

/// The struct to determine how the Scanner store should look like
pub struct Scanners {
    /// The animations for all scanners
    /// all vecs are left to right when looking towards the stage
    /// Animation< Scanner< Position< x_position, y_position, up, down, in, out >>>
    /// the up, down, in, out bools indicate, if the scanner should blackout(false) or light(true) when moving to that coordinates
    animations: Vec<Vec<Vec<(u8, u8, bool, bool, bool, bool)>>>,
    /// Stores the index of the current animation
    active_animation: u8,
    /// stores a single byte representation of the current color of the scanners
    current_color: u8,
    /// scanners should light up if this is true and the current animations direction is up
    light_mode_up: bool,
    /// scanners should light up if this is true and the current animations direction is down
    light_mode_down: bool,
    /// scanners should light up if this is true and the current animations direction is in
    light_mode_in: bool,
    /// scanners should light up if this is true and the current animations direction is out
    light_mode_out: bool,
    /// This vec contains a bool for every scanner. true = normal behavior, false = blackout
    enabled_scanners: Vec<bool>,
    /// the animation index. This should count into oblivion on each render cyncle
    index: u64
}

/// This is responsible for storing the current scanner state and how they should react to certain situations
impl Scanners {
    /// This creates, fills and returns the Scanner object
    pub fn new(dmx_config_store: &DmxConfigStore) -> Scanners {
        let animations = Scanners::read_scanner_animation_files(dmx_config_store);

        let mut enabled_scanners = vec!();
        for _ in 0..dmx_config_store.get_scanner_count() {
            enabled_scanners.push(true);
        }

        Scanners {
            animations: animations,
            active_animation: 0,
            current_color: 0,
            light_mode_up: true,
            light_mode_down: true,
            light_mode_in: false,
            light_mode_out: false,
            enabled_scanners:  enabled_scanners,
            index: 0
        }
    }
    /// reads the scanner animation files and returns the constructed animation vec
    fn read_scanner_animation_files(dmx_config_store: &DmxConfigStore) -> Vec<Vec<Vec<(u8, u8, bool, bool, bool, bool)>>> {
        let mut animations: Vec<Vec<Vec<(u8, u8, bool, bool, bool, bool)>>> = vec!();
        let scanner_count = dmx_config_store.get_scanner_count();
        for animation_i in 0..dmx_config_store.get_scanner_animation_count() {
            animations.push(vec!());
            for _ in 0..scanner_count {
                animations[animation_i as usize].push(vec!());
            }
        }

        animations[0] = Scanners::help_read_scanner_animation_files("snake.tpl", scanner_count);

        animations
    }
    /// helper function for read_scanner_animation_files <br>
    /// returns the positions for each scanner from a given file for x (scanner_count) scanner
    fn help_read_scanner_animation_files(animation_file_name: &str, scanner_count: u8) -> Vec<Vec<(u8, u8, bool, bool, bool, bool)>> {
        let mut scanner_return_vec: Vec<Vec<(u8, u8, bool, bool, bool, bool)>> = vec!();
        for _ in 0..scanner_count {
            scanner_return_vec.push(vec!());
        }
        let mut position_i: isize = -1;
        let mut plain_content = match std::fs::read_to_string(Path::new((String::from("src/scanners/") + animation_file_name).as_str())) {
            Ok(e) => e,
            Err(e) => {
                logger::log("Error occured while reading animation file");
                panic!("Error occured while reading animation file {}\n", e);
            }
        };
        plain_content = plain_content.replace(" ", "");
        let vec_content = plain_content.split("\n");
        for (line_i, line) in vec_content.clone().enumerate() {
            if line_i == 0 || line_i % 6 == 0 { position_i += 1; continue; }

            for scanner_i in 0..scanner_count {
                scanner_return_vec[scanner_i as usize].push((0, 0, false, false, false, false));
            }
            let mut line_params = line.split(",");
            let x: u8 = line_params.next().unwrap().parse().unwrap();
            let y: u8 = line_params.next().unwrap().parse().unwrap();
            let dir_up: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };
            let dir_down: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };
            let dir_in: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };
            let dir_out: bool = if line_params.next().unwrap().parse::<u8>().unwrap() == 1 { true } else { false };

            scanner_return_vec[(line_i % 6) - 1 as usize][position_i as usize].0 = x;
            scanner_return_vec[(line_i % 6) - 1 as usize][position_i as usize].1 = y;
            scanner_return_vec[(line_i % 6) - 1 as usize][position_i as usize].2 = dir_up;
            scanner_return_vec[(line_i % 6) - 1 as usize][position_i as usize].3 = dir_down;
            scanner_return_vec[(line_i % 6) - 1 as usize][position_i as usize].4 = dir_in;
            scanner_return_vec[(line_i % 6) - 1 as usize][position_i as usize].5 = dir_out;
        }
        scanner_return_vec
    }
    /// Increments the animation index <br>
    /// Returns the target coords and dimmer states for all scanners like get_current_position()
    pub fn trigger_next_step(&mut self, dmx_renderer: &mut DmxRenderer) -> Vec<(u8, u8, bool)> {
        self.index += 1;
        dmx_renderer.set_updateable(None);
        self.get_current_position()
    }
    /// returns the target coords and dimmer states for all scanners by filtering the active_animation variable with the current animation index
    pub fn get_current_position(&self) -> Vec<(u8, u8, bool)> {
        let mut result_vec: Vec<(u8, u8, bool)> = vec!();
        let scanner_count = self.animations[0].len();
        for i in 0..scanner_count {
            result_vec.push((0, 0, false));
            result_vec[i].0 = self.animations[self.active_animation as usize][i as usize][self.index as usize % (self.animations[self.active_animation as usize][0].len() / scanner_count)].0;
            result_vec[i].1 = self.animations[self.active_animation as usize][i as usize][self.index as usize % (self.animations[self.active_animation as usize][0].len() / scanner_count)].1;

            // calculate if lit
            if self.animations[self.active_animation as usize][i as usize][self.index as usize % (self.animations[self.active_animation as usize][0].len() / scanner_count)].2 && !self.light_mode_up {
                result_vec[i].2 = true;
            } else if self.animations[self.active_animation as usize][i as usize][self.index as usize % (self.animations[self.active_animation as usize][0].len() / scanner_count)].3 && !self.light_mode_down {
                result_vec[i].2 = true;
            } else if self.animations[self.active_animation as usize][i as usize][self.index as usize % (self.animations[self.active_animation as usize][0].len() / scanner_count)].4 && !self.light_mode_in {
                result_vec[i].2 = true;
            } else if self.animations[self.active_animation as usize][i as usize][self.index as usize % (self.animations[self.active_animation as usize][0].len() / scanner_count)].5 && !self.light_mode_out {
                result_vec[i].2 = true;
            }
        }
        result_vec
    }
    /// Returns the current one byte representation of the current color of the scanners
    pub fn get_current_color(&self) -> &u8 {
        &self.current_color
    }
    /// sets the current color by processing the given three bytes into one <br>
    /// Available colors by the here used JB Systems LED Victory Scan are <br>
    /// WHITE       0 <br>
    /// YELLOW      8 <br>
    /// BLUE       38 <br>
    /// GREEN      68 <br>
    /// RED        98 <br>
    /// PURPLE    128 <br>
    /// DARK_BLUE 158 <br>
    /// ORANGE    188 <br>
    /// PINK      218
    pub fn set_current_color(&mut self, color: &(f32, f32, f32)) {
        if color.0 == color.1 && color.0 == color.2 {
            self.current_color = 0;
        } else
        if color.0 > 200.0 && color.1 < 30.0 && color.2 < 30.0 {
            self.current_color = 98;
        } else
        if color.0 < 30.0 && color.1 > 200.0 && color.2 < 30.0 {
            self.current_color = 68;
        } else
        if color.0 < 30.0 && color.1 < 30.0 && color.2 > 200.0 {
            self.current_color = 38;
        } else
        if color.0 > color.2 && color.1 > color.2 {
            self.current_color = 8;
        } else
        if color.0 > color.1 && color.2 > color.1 {
            self.current_color = 128;
        } else
        if color.0 > color.1 + 30.0 && color.1 > color.2 + 100.0 {
            self.current_color = 188;
        } else
        if color.0 > color.2 + 30.0 && color.2 > color.1 + 100.0 {
            self.current_color = 218;
        } else {
            self.current_color = 0;
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
    /// Returns the enabled_scanner variable
    pub fn get_enabled_scanner(&self) -> &Vec<bool> {
        &self.enabled_scanners
    }
}