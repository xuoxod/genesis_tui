use genesis_tui::utils::color::RgbLerp;
use genesis_tui::utils::projection::ViewportProjection;
use glam::Vec3;

#[test]
fn test_color_lerping_and_queue() {
    // Start at purely White
    let mut color = RgbLerp::new(Vec3::new(255.0, 255.0, 255.0), 0.5); // 50% step per tick

    // Target pure Red
    color.push_target(Vec3::new(255.0, 0.0, 0.0));

    // Step 1: Should be 50% between White and Red
    color.step();
    assert_eq!(color.current, Vec3::new(255.0, 127.5, 127.5));

    // Step 2... n until it reaches the target
    for _ in 0..10 {
        color.step();
    }

    // Should have snapped and cleared the queue
    assert_eq!(color.current, Vec3::new(255.0, 0.0, 0.0));
    assert!(color.targets.is_empty());
}

#[test]
fn test_viewport_projection() {
    let sim_pos = ViewportProjection::term_to_grid(
        50, 25, // Clicked middle of term
        100, 50, // Term is 100x50 UI cells
        200.0, 100.0, // Grid is 200x100 f32
    );

    assert_eq!(sim_pos.x, 100.0); // Exact X middle
    assert_eq!(sim_pos.y, 50.0); // Exact Y middle (inverted logic accounted for)
}

// Gradient Specific Tests
use genesis_tui::utils::gradient::generate_gradient;

#[test]
fn test_generic_gradient_generation() {
    let waypoints = vec![
        Vec3::new(255.0, 0.0, 0.0), // RED
        Vec3::new(0.0, 255.0, 0.0), // GREEN
        Vec3::new(0.0, 0.0, 255.0), // BLUE
    ];

    let discrete = generate_gradient(&waypoints, 3);
    // Over exactly 3 steps, it should snap directly to the exact waypoints
    assert_eq!(discrete.len(), 3);
    assert_eq!(discrete[0], waypoints[0]);
    assert!(
        discrete[1].distance(waypoints[1]) < 0.1,
        "Middle color drift"
    );
    assert_eq!(discrete[2], waypoints[2]);

    let smooth = generate_gradient(&waypoints, 100);
    // Smooth transitions across 100 ticks
    assert_eq!(smooth.len(), 100);
    assert_eq!(smooth[0], waypoints[0], "Starts at RED");
    assert_eq!(smooth[99], waypoints[2], "Ends at BLUE");
}
