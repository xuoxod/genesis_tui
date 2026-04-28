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
        Self { position, mass, spawn_tick, duration }
    }
    
    pub fn is_active(&self, current_tick: usize) -> bool {
        current_tick - self.spawn_tick < self.duration
    }
}
