use crate::core::grid::Grid;
use crate::core::entity::Entity;

pub struct Engine {
    grid: Grid,
    entities: Vec<Entity>,
    epoch: u64,
}

impl Engine {
    pub fn new(grid: Grid) -> Self { Self { grid, entities: Vec::new(), epoch: 0 } }
    pub fn epoch(&self) -> u64 { self.epoch }
    pub fn entities(&self) -> &[Entity] { &self.entities }
    pub fn grid(&self) -> &Grid { &self.grid }
    pub fn add_entity(&mut self, e: Entity) { self.entities.push(e); }
    pub fn tick(&mut self) { self.epoch += 1; }
}
