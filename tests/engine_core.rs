use genesis_tui::core::engine::Engine;
use genesis_tui::core::grid::{Grid, Position};
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;

#[test]
fn test_engine_time_controls() {
    let mut engine = Engine::new(Grid::new(100, 100));
    let entity = Entity::new(1, Position::new(50.0, 50.0), Genome::new_random(16));
    engine.add_entity(entity);

    // Pause test
    engine.toggle_pause();
    engine.tick();
    assert_eq!(engine.tick_count(), 0, "No time jump while paused");

    // Frame-by-frame stepping
    engine.step_forward();
    assert_eq!(engine.tick_count(), 1, "Manual step bypasses pause");

    // Time inversion (Reverse)
    engine.step_forward(); // Goes to 2
    assert_eq!(engine.tick_count(), 2);
    engine.step_backward(); 
    assert_eq!(engine.tick_count(), 1, "Time travels backward");
    assert_eq!(engine.entities().len(), 1, "Fossil record restores entity state");
}

#[test]
fn test_engine_reset() {
    let mut engine = Engine::new(Grid::new(100, 100));
    engine.step_forward();
    engine.add_entity(Entity::new(1, Position::new(0.0, 0.0), Genome::new_random(16)));
    engine.reset();
    assert_eq!(engine.tick_count(), 0);
    assert_eq!(engine.entities().len(), 0);
}

use genesis_tui::core::grid::Velocity;
use genesis_tui::utils::fence::FenceSide;

#[test]
fn test_engine_electric_fence_collision() {
    let grid = Grid::new(100, 100);
    let mut engine = Engine::new(grid);
    
    // Entity moving right, at the edge
    let mut entity = Entity::new(1, Position::new(99.0, 50.0), Genome::new_random(16));
    entity.set_velocity(Velocity::new(5.0, 0.0)); // moving right
    engine.add_entity(entity);
    
    // Turn on the right fence
    engine.fence_mut().turn_on(FenceSide::Right);
    
    engine.step_forward();
    engine.step_forward(); // needs two ticks because bounds check happens before translation
    
    let e = &engine.entities()[0];
    assert!(e.is_electrified(), "Entity must be electrified after hitting an active electric fence");
    
    // Now test a safe wall
    let mut engine_safe = Engine::new(Grid::new(100, 100));
    let mut entity2 = Entity::new(2, Position::new(1.0, 50.0), Genome::new_random(16));
    entity2.set_velocity(Velocity::new(-5.0, 0.0)); // moving left
    engine_safe.add_entity(entity2);
    
    // Left fence is OFF by default
    engine_safe.step_forward();
    engine_safe.step_forward(); // hits the safe left wall
    
    let e2 = &engine_safe.entities()[0];
    assert!(!e2.is_electrified(), "Entity must NOT be electrified if the fence is off");
}
