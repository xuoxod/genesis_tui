use genesis_tui::utils::physics::gravitational_pull;
use genesis_tui::core::grid::{Position, Velocity};

#[test]
fn test_gravitational_pull() {
    let source = Position::new(10.0, 10.0); // The singularity
    let target = Position::new(0.0, 10.0);  // The entity (left of singularity)

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
