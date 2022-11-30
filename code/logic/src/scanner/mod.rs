use crate::dmx_renderer::DmxRenderer;
use crate::config_store::DmxConfigStore;

pub struct Scanner {
    // * all vecs are left to right when looking towards the stage
    // * Animation<Scanner<Position<x_position, y_position, direction_to_know_when_to_light_up_up, down, in, out
    animations: Vec<Vec<Vec<(u8, u8, bool, bool, bool, bool)>>>,
    active_animation: u8,
    current_color: u8,
    light_mode_up: bool,
    light_mode_down: bool,
    light_mode_in: bool,
    light_mode_out: bool,
    enabled_scanner: Vec<bool>,
    index: u64
}

impl Scanner {
    pub fn new(dmx_config_store: &DmxConfigStore) -> Scanner {
        let mut animations: Vec<Vec<Vec<(u8, u8, bool, bool, bool, bool)>>> = vec!();
        for _ in 0..1 {
            animations.push(vec!());
        }
        // Todo: develop custom file format for this to not clog up this file
        // testanimation1 = snake | only two scanner are implemented for this.
        for i in 0..dmx_config_store.get_scanner_count() {
            animations[0].push(vec!());
            for _ in 0..4 {
                animations[0][i as usize].push((0,0,false,false,false,false));
            }
        }
        // position1
        animations[0][0][0].0 = 50;
        animations[0][0][0].1 = 50;
        animations[0][0][0].2 = false;
        animations[0][0][0].3 = false;
        animations[0][0][0].4 = true;
        animations[0][0][0].5 = true;

        // position2
        animations[0][0][1].0 = 50;
        animations[0][0][1].1 = 200;
        animations[0][0][1].2 = true;
        animations[0][0][1].3 = false;
        animations[0][0][1].4 = false;
        animations[0][0][1].5 = false;

        // position3
        animations[0][0][2].0 = 200;
        animations[0][0][2].1 = 200;
        animations[0][0][2].2 = false;
        animations[0][0][2].3 = false;
        animations[0][0][2].4 = true;
        animations[0][0][2].5 = true;

        // position4
        animations[0][0][3].0 = 200;
        animations[0][0][3].1 = 50;
        animations[0][0][3].2 = false;
        animations[0][0][3].3 = true;
        animations[0][0][3].4 = false;
        animations[0][0][3].5 = false;

        let mut enabled_scanner = vec!();
        for _ in 0..dmx_config_store.get_scanner_count() {
            enabled_scanner.push(true);
        }

        Scanner {
            animations: animations,
            active_animation: 0,
            current_color: 0,
            light_mode_up: true,
            light_mode_down: true,
            light_mode_in: false,
            light_mode_out: false,
            enabled_scanner:  enabled_scanner,
            index: 0
        }
    }
    pub fn trigger_next_step(&mut self, dmx_renderer: &mut DmxRenderer) {
        self.index += 1;
        dmx_renderer.set_updateable(None);
    }
    pub fn get_current_position(&self) -> Vec<(u8, u8, bool)> {
        let mut result_vec: Vec<(u8, u8, bool)> = vec!();
        let scanner_count = self.animations[0].len();
        for i in 0..scanner_count {
            result_vec.push((0, 0, false));
            result_vec[i].0 = self.animations[self.active_animation as usize][i as usize][self.index as usize % self.animations[self.active_animation as usize][i as usize].len()].0;
            result_vec[i].1 = self.animations[self.active_animation as usize][i as usize][self.index as usize % self.animations[self.active_animation as usize][i as usize].len()].1;

            // calculate if lit
            if self.animations[self.active_animation as usize][i as usize][self.index as usize % self.animations[self.active_animation as usize][i as usize].len()].2 && self.light_mode_up {
                result_vec[i].2 = true;
            } else if self.animations[self.active_animation as usize][i as usize][self.index as usize % self.animations[self.active_animation as usize][i as usize].len()].3 && self.light_mode_down {
                result_vec[i].2 = true;
            } else if self.animations[self.active_animation as usize][i as usize][self.index as usize % self.animations[self.active_animation as usize][i as usize].len()].4 && self.light_mode_in {
                result_vec[i].2 = true;
            } else if self.animations[self.active_animation as usize][i as usize][self.index as usize % self.animations[self.active_animation as usize][i as usize].len()].5 && self.light_mode_out {
                result_vec[i].2 = true;
            }
        }
        result_vec
    }
    pub fn get_current_color(&self) -> &u8 {
        &self.current_color
    }
    pub fn set_current_color(&mut self, color: &(f32, f32, f32)) {
        // Todo: construct color 
        self.current_color = if color.0 > color.1 {
            98
        } else {
            8
        }
    }
    pub fn set_current_light_mode_up(&mut self, light_mode_up: Option<bool>) {
        match light_mode_up {
            Some(e) => self.light_mode_up = e,
            None => self.light_mode_up = !self.light_mode_up
        }
    }
    pub fn set_current_light_mode_down(&mut self, light_mode_down: Option<bool>) {
        match light_mode_down {
            Some(e) => self.light_mode_down = e,
            None => self.light_mode_down = !self.light_mode_down
        }
    }
    pub fn set_current_light_mode_in(&mut self, light_mode_in: Option<bool>) {
        match light_mode_in {
            Some(e) => self.light_mode_in = e,
            None => self.light_mode_in = !self.light_mode_in
        }
    }
    pub fn set_current_light_mode_out(&mut self, light_mode_out: Option<bool>) {
        match light_mode_out {
            Some(e) => self.light_mode_out = e,
            None => self.light_mode_out = !self.light_mode_out
        }
    }
    pub fn get_enabled_scanner(&self) -> &Vec<bool> {
        &self.enabled_scanner
    }
}