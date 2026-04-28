use genesis_tui::core::grid::{Position, Velocity};
use genesis_tui::utils::physics::gravitational_pull;

#[test]
fn test_gravitational_pull() {
    let source = Position::new(10.0, 10.0); // The singularity
    let target = Position::new(0.0, 10.0); // The entity (left of singularity)

    let pull = gravitational_pull(&source, &target, 100.0, 5.0);

    // Target is exactly 10 units away on the X axis. dist_sq = 100. force = 100 / 100 = 1.0
    // Vector should be purely in the positive X direction (towards singularity).
    assert!(pull.x > 0.99 && pull.x < 1.01);
    assert_eq!(pull.y, 0.0);

    // Test max force clamping
    let target_close = Position::new(8.0, 10.0); // 2 units away, dist_sq = 4. force = 100/4 = 25 -> clamp to 5.0
    let pull_close = gravitational_pull(&source, &target_close, 100.0, 5.0);
    assert_eq!(pull_close.x, 5.0);
}

#[test]
fn test_ring_intersection() {
    let center = Position::new(0.0, 0.0);
    let target = Position::new(3.0, 4.0); // Exact distance = 5.0

    // Radius requires direct hit within tolerance
    assert!(genesis_tui::utils::physics::ring_intersection(
        &target, &center, 5.0, 0.1
    ));

    // Target is outside the ring boundary
    assert!(!genesis_tui::utils::physics::ring_intersection(
        &target, &center, 6.0, 0.1
    ));
}

#[test]
fn test_boids_flocking_math() {
    let target_pos = Position::new(10.0, 10.0);
    let target_vel = Velocity::new(1.0, 0.0);

    let neighbors = vec![
        (Position::new(10.0, 12.0), Velocity::new(1.0, 0.0)),
        (Position::new(12.0, 10.0), Velocity::new(0.0, 1.0)),
    ];

    // Cohesion: Center of mass = (11, 11). Target is at (10, 10). Direction towards COM is (+1, +1).
    let cohesion_dir = genesis_tui::utils::physics::boids_cohesion(&target_pos, &neighbors);
    assert!(cohesion_dir.x > 0.0 && cohesion_dir.y > 0.0);

    // Alignment: Average velocity = (0.5, 0.5). Target is (1, 0).
    // Desired change in velocity = (-0.5, +0.5).
    let alignment_dir = genesis_tui::utils::physics::boids_alignment(&target_vel, &neighbors);
    assert!(alignment_dir.x < 0.0 && alignment_dir.y > 0.0);

    // Separation:
    // From (10, 12): vector away is (0, -2).
    // From (12, 10): vector away is (-2, 0).
    // Total separation = (-2, -2).
    let separation_dir =
        genesis_tui::utils::physics::boids_separation(&target_pos, &neighbors, 5.0);
    assert!(separation_dir.x < 0.0 && separation_dir.y < 0.0);
}
