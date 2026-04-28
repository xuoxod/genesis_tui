use genesis_tui::core::engine::Engine;
use genesis_tui::core::grid::{Grid, Position};
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;

#[test]
fn test_engine_time_controls() {
    let mut engine = Engine::new(Grid::new(100, 100));
    let entity = Entity::new(1, Position::new(50, 50), Genome::new_random(16));
    engine.add_entity(entity);

    // Pause test
    engine.toggle_pause();
    engine.tick();
    assert_eq!(engine.epoch(), 0, "No time jump while paused");

    // Frame-by-frame stepping
    engine.step_forward();
    assert_eq!(engine.epoch(), 1, "Manual step bypasses pause");

    // Time inversion (Reverse)
    engine.step_forward(); // Goes to 2
    assert_eq!(engine.epoch(), 2);
    engine.step_backward(); 
    assert_eq!(engine.epoch(), 1, "Time travels backward");
    assert_eq!(engine.entities().len(), 1, "Fossil record restores entity state");
}

#[test]
fn test_engine_reset() {
    let mut engine = Engine::new(Grid::new(100, 100));
    engine.step_forward();
    engine.add_entity(Entity::new(1, Position::new(0,0), Genome::new_random(16)));
    engine.reset();
    assert_eq!(engine.epoch(), 0);
    assert_eq!(engine.entities().len(), 0);
}
