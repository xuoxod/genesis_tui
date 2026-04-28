use genesis_tui::core::engine::Engine;
use genesis_tui::core::grid::{Grid, Position};
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;

#[test]
fn test_engine_initialization() {
    let grid = Grid::new(100, 100);
    let engine = Engine::new(grid);
    assert_eq!(engine.epoch(), 0);
    assert_eq!(engine.entities().len(), 0);
}

#[test]
fn test_engine_tick() {
    let grid = Grid::new(100, 100);
    let mut engine = Engine::new(grid);
    let entity = Entity::new(1, Position::new(50, 50), Genome::new_random(16));
    engine.add_entity(entity);
    assert_eq!(engine.entities().len(), 1);
    engine.tick();
    assert_eq!(engine.epoch(), 1);
}
