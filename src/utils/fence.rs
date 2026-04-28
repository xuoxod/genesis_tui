use glam::Vec3;
use crate::constants::colors;
use crate::utils::gradient::generate_gradient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct ElectricFence {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
    // Voltage color array over time!
    spark_gradient: Vec<Vec3>,
}

impl ElectricFence {
    pub fn new() -> Self {
        // High voltage electric blue, cyan, and glowing white effects!
        let gradient_stops = vec![colors::NAVY, colors::CYAN, colors::WHITE, colors::CYAN, colors::NAVY];
        let spark_gradient = generate_gradient(&gradient_stops, 30); // 30-tick effect cycle
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            spark_gradient,
        }
    }

    pub fn is_active(&self, side: FenceSide) -> bool {
        match side {
            FenceSide::Top => self.top,
            FenceSide::Bottom => self.bottom,
            FenceSide::Left => self.left,
            FenceSide::Right => self.right,
        }
    }

    pub fn turn_on(&mut self, side: FenceSide) {
        self.set_side(side, true);
    }
    
    pub fn turn_off(&mut self, side: FenceSide) {
        self.set_side(side, false);
    }

    pub fn turn_on_all(&mut self) {
        self.top = true;
        self.bottom = true;
        self.left = true;
        self.right = true;
    }

    pub fn turn_off_all(&mut self) {
        self.top = false;
        self.bottom = false;
        self.left = false;
        self.right = false;
    }

    fn set_side(&mut self, side: FenceSide, state: bool) {
        match side {
            FenceSide::Top => self.top = state,
            FenceSide::Bottom => self.bottom = state,
            FenceSide::Left => self.left = state,
            FenceSide::Right => self.right = state,
        }
    }

    /// Provides cool electrified ASCII characters mapping voltage state over time
    pub fn get_effect(&self, side: FenceSide, tick: usize) -> Option<(&'static str, Vec3)> {
        if !self.is_active(side) {
            return None;
        }
        
        let cycle_index = tick % self.spark_gradient.len();
        let color = self.spark_gradient[cycle_index];
        
        // A series of nerdy electrified chars that animate over time!
        let chars = ["|", "/", "-", "\\", "++", "#", "X", ">", "<", "~", "*"];
        let char_idx = (tick / 3) % chars.len();
        
        Some((chars[char_idx], color))
    }
}
