use crate::core::grid::Position;
use crate::core::genetics::Genome;

#[derive(Debug, Clone)]
pub struct Entity {
    id: u64,
    position: Position,
    genome: Genome,
}

impl Entity {
    pub fn new(id: u64, position: Position, genome: Genome) -> Self {
        Self { id, position, genome }
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn position(&self) -> &Position { &self.position }
    pub fn genome(&self) -> &Genome { &self.genome }
    
    pub fn mutate_genome(&mut self, rate: f64) {
        self.genome.mutate(rate);
    }
}
