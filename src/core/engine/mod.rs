use crate::core::grid::Grid;
use crate::core::entity::Entity;

pub struct Engine {
    grid: Grid,
    entities: Vec<Entity>,
    epoch: u64,
    is_paused: bool,
    fossil_record: Vec<(u64, Vec<Entity>)>,
}

impl Engine {
    pub fn new(grid: Grid) -> Self { 
        Self { grid, entities: Vec::new(), epoch: 0, is_paused: false, fossil_record: Vec::new() } 
    }
    
    pub fn epoch(&self) -> u64 { self.epoch }
    pub fn entities(&self) -> &[Entity] { &self.entities }
    pub fn grid(&self) -> &Grid { &self.grid }
    pub fn is_paused(&self) -> bool { self.is_paused }
    
    pub fn add_entity(&mut self, e: Entity) { self.entities.push(e); }
    
    pub fn toggle_pause(&mut self) { self.is_paused = !self.is_paused; }
    
    pub fn reset(&mut self) {
        self.epoch = 0;
        self.entities.clear();
        self.fossil_record.clear();
    }

    pub fn tick(&mut self) {
        if !self.is_paused { self.step_forward(); }
    }

    pub fn step_forward(&mut self) {
        if self.fossil_record.len() >= 1000 { self.fossil_record.remove(0); }
        self.fossil_record.push((self.epoch, self.entities.clone()));
        self.epoch += 1;
    }

    pub fn step_backward(&mut self) {
        if let Some((prev_epoch, prev_ent)) = self.fossil_record.pop() {
            self.epoch = prev_epoch;
            self.entities = prev_ent;
        }
    }
}
