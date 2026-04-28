use crate::core::grid::{Position, Velocity};
use crate::core::genetics::Genome;

#[derive(Debug, Clone)]
pub struct Entity {
    id: u64,
    position: Position,
    velocity: Velocity,
    genome: Genome,
}

impl Entity {
    pub fn new(id: u64, position: Position, genome: Genome) -> Self {
        // We initialize with a dead velocity and let the engine inject force, or derive it from the genome.
        Self { id, position, velocity: Velocity::ZERO, genome }
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn position(&self) -> &Position { &self.position }
    pub fn velocity(&self) -> &Velocity { &self.velocity }
    pub fn genome(&self) -> &Genome { &self.genome }
    
    pub fn set_position(&mut self, pos: Position) { self.position = pos; }
    pub fn set_velocity(&mut self, vel: Velocity) { self.velocity = vel; }

    pub fn mutate_genome(&mut self, rate: f64) {
        self.genome.mutate(rate);
    }
}
