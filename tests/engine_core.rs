use genesis_tui::core::engine::Engine;
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;
use genesis_tui::core::grid::{Grid, Position};

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
    assert_eq!(
        engine.entities().len(),
        1,
        "Fossil record restores entity state"
    );
}

#[test]
fn test_engine_reset() {
    let mut engine = Engine::new(Grid::new(100, 100));
    engine.step_forward();
    engine.add_entity(Entity::new(
        1,
        Position::new(0.0, 0.0),
        Genome::new_random(16),
    ));
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
    assert!(
        e.is_electrified(),
        "Entity must be electrified after hitting an active electric fence"
    );

    // Now test a safe wall
    let mut engine_safe = Engine::new(Grid::new(100, 100));
    let mut entity2 = Entity::new(2, Position::new(1.0, 50.0), Genome::new_random(16));
    entity2.set_velocity(Velocity::new(-5.0, 0.0)); // moving left
    engine_safe.add_entity(entity2);

    // Left fence is OFF by default
    engine_safe.step_forward();
    engine_safe.step_forward(); // hits the safe left wall

    let e2 = &engine_safe.entities()[0];
    assert!(
        !e2.is_electrified(),
        "Entity must NOT be electrified if the fence is off"
    );
}

#[test]
fn test_engine_mouse_interaction_handling() {
    let mut engine = Engine::new(Grid::new(100, 100));
    let mut ent = Entity::new(1, Position::new(50.0, 50.0), Genome::new_random(16));
    ent.set_velocity(Velocity::new(0.0, 0.0));
    engine.add_entity(ent);

    engine.handle_click(51.0, 51.0); // Within radius
    engine.handle_click(10.0, 10.0); // Far away

    let clicked_ent = &engine.entities()[0];

    // Within 25 dist_sq, distance here is ~2.0, so it registers
    assert!(clicked_ent
        .get_render_effect(engine.tick_count() as usize)
        .is_some());
    assert_ne!(
        clicked_ent.velocity().x,
        0.0,
        "Velocity should have been nudged"
    );
}

#[test]
fn test_engine_add_remove_entities() {
    let mut engine =
        genesis_tui::core::engine::Engine::new(genesis_tui::core::grid::Grid::new(100, 100));
    engine.reset(); // clear
    assert_eq!(engine.entities().len(), 0);

    // Add dynamically
    engine.add_entities(5);
    assert_eq!(engine.entities().len(), 5);

    // Remove dynamically
    engine.remove_entities(3);
    assert_eq!(engine.entities().len(), 2);

    // Remove more than exists
    engine.remove_entities(10);
    assert_eq!(engine.entities().len(), 0);
}
