cat << 'TEST_CODE' >> tests/engine_core.rs

#[test]
fn test_engine_pause() {
    let grid = Grid::new(100, 100);
    let mut engine = Engine::new(grid);
    assert!(!engine.is_paused());
    engine.tick();
    assert_eq!(engine.epoch(), 1);
    engine.toggle_pause();
    assert!(engine.is_paused());
    engine.tick();
    assert_eq!(engine.epoch(), 1, "Epoch should not advance while paused");
    engine.toggle_pause();
    assert!(!engine.is_paused());
    engine.tick();
    assert_eq!(engine.epoch(), 2);
}
TEST_CODE

cat << 'MOD_CODE' > src/core/engine/mod.rs
use crate::core::grid::Grid;
use crate::core::entity::Entity;

pub struct Engine {
    grid: Grid,
    entities: Vec<Entity>,
    epoch: u64,
    is_paused: bool,
}

impl Engine {
    pub fn new(grid: Grid) -> Self { 
        Self { grid, entities: Vec::new(), epoch: 0, is_paused: false } 
    }
    pub fn epoch(&self) -> u64 { self.epoch }
    pub fn entities(&self) -> &[Entity] { &self.entities }
    pub fn grid(&self) -> &Grid { &self.grid }
    pub fn is_paused(&self) -> bool { self.is_paused }
    pub fn add_entity(&mut self, e: Entity) { self.entities.push(e); }
    pub fn tick(&mut self) { 
        if self.is_paused { return; }
        self.epoch += 1; 
    }
    pub fn reset(&mut self) {
        self.epoch = 0;
        self.entities.clear();
        self.is_paused = false;
    }
    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }
}
MOD_CODE
