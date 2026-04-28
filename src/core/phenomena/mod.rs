use crate::core::grid::Position;

#[derive(Clone, Debug)]
pub struct Singularity {
    pub position: Position,
    pub mass: f32,
    pub spawn_tick: usize,
    pub duration: usize,
}

impl Singularity {
    pub fn new(position: Position, mass: f32, spawn_tick: usize, duration: usize) -> Self {
        Self {
            position,
            mass,
            spawn_tick,
            duration,
        }
    }

    pub fn is_active(&self, current_tick: usize) -> bool {
        current_tick - self.spawn_tick < self.duration
    }
}

#[derive(Clone, Debug)]
pub struct RadarPing {
    pub position: Position,
    pub spawn_tick: usize,
    pub speed: f32,
    pub max_radius: f32,
}

impl RadarPing {
    pub fn new(position: Position, spawn_tick: usize, speed: f32, max_radius: f32) -> Self {
        Self {
            position,
            spawn_tick,
            speed,
            max_radius,
        }
    }

    pub fn current_radius(&self, current_tick: usize) -> f32 {
        let elapsed = current_tick.saturating_sub(self.spawn_tick) as f32;
        elapsed * self.speed
    }

    pub fn is_active(&self, current_tick: usize) -> bool {
        self.current_radius(current_tick) <= self.max_radius
    }
}
