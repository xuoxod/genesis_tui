use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;
use genesis_tui::core::grid::{Position, Velocity};
use genesis_tui::core::visuals::Shape;
use glam::Vec3;

#[test]
fn test_entity_shape_and_color_independence() {
    let mut entity = Entity::new(1, Position::new(0.0, 0.0), Genome::new_random(10));

    // Default assertions
    assert_eq!(entity.shape_char(), "●");

    // Change shape
    entity.set_shape(Shape::Vortex);
    assert_eq!(entity.shape_char(), "🌀");

    // Change base color
    let new_color = Vec3::new(255.0, 0.0, 0.0); // Red
    entity.set_base_color(new_color);
    assert_eq!(entity.base_color(), new_color);

    // Reset to normal
    entity.reset_visuals();
    assert_eq!(entity.shape_char(), "●"); // Resets to dot
}

#[test]
fn test_entity_gradient_effects() {
    let mut entity = Entity::new(1, Position::new(0.0, 0.0), Genome::new_random(10));

    // Apply custom generic gradient
    let my_gradient = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(255.0, 255.0, 255.0)];
    entity.apply_custom_gradient(my_gradient.clone());

    assert!(entity.has_custom_gradient());

    entity.reset_visuals();
    assert!(!entity.has_custom_gradient());
}
