use genesis_tui::utils::fence::{ElectricFence, FenceSide};

#[test]
fn test_fence_toggles_and_sides() {
    let mut fence = ElectricFence::new();
    assert_eq!(fence.is_active(FenceSide::Top), false);

    fence.turn_on_all();
    assert_eq!(fence.is_active(FenceSide::Top), true);
    assert_eq!(fence.is_active(FenceSide::Bottom), true);
    assert_eq!(fence.is_active(FenceSide::Left), true);
    assert_eq!(fence.is_active(FenceSide::Right), true);

    fence.turn_off(FenceSide::Top);
    assert_eq!(fence.is_active(FenceSide::Top), false);
    assert_eq!(fence.is_active(FenceSide::Bottom), true);
}

#[test]
fn test_fence_electrified_effects() {
    let mut fence = ElectricFence::new();
    fence.turn_on(FenceSide::Right);

    // Simulate some ticks to see the effect state change
    let effect_t0 = fence.get_effect(FenceSide::Right, 0);
    let effect_t10 = fence.get_effect(FenceSide::Right, 10);

    // Should vary based on tick
    assert!(effect_t0.is_some());
    assert!(effect_t10.is_some());

    let top_effect = fence.get_effect(FenceSide::Top, 0);
    assert!(top_effect.is_none()); // because it is OFF
}
