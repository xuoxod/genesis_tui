use genesis_tui::core::engine::Engine;
use genesis_tui::core::grid::{Grid, Position, Velocity};
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;

#[test]
fn test_newtonian_kinematics() {
    let mut engine = Engine::new(Grid::new(100, 100));
    
    let mut entity = Entity::new(1, Position::new(50.0, 50.0), Genome::new_random(16));
    entity.set_velocity(Velocity::new(1.0, -1.0));
    engine.add_entity(entity);

    engine.step_forward();

    // After 1 tick, pos should be 51, 49
    let pos = engine.entities()[0].position();
    assert_eq!(pos.x, 51.0);
    assert_eq!(pos.y, 49.0);
}

#[test]
fn test_elastic_boundary_collision() {
    let mut engine = Engine::new(Grid::new(100, 100));
    
    // Spawn exactly on the boundary, forcing it past
    let mut entity = Entity::new(1, Position::new(100.0, 100.0), Genome::new_random(16));
    entity.set_velocity(Velocity::new(2.0, 2.0));
    engine.add_entity(entity);

    engine.step_forward();

    // Expected: Hard bounce against the wall, reversing velocity
    let pos = engine.entities()[0].position();
    let vel = engine.entities()[0].velocity();
    
    assert!(pos.x <= 100.0, "Entity breached X boundary!");
    assert!(pos.y <= 100.0, "Entity breached Y boundary!");
    assert_eq!(vel.x, -2.0, "X Velocity did not reverse");
    assert_eq!(vel.y, -2.0, "Y Velocity did not reverse");
}
