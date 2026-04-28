use genesis_tui::utils::fx::animate_frame;
use genesis_tui::constants::effects;

#[test]
fn test_animate_frame_wraps_correctly() {
    let dummy_frames = &["A", "B", "C"];
    
    // tick 0 -> A, tick 1 -> B, tick 2 -> C, tick 3 -> A
    assert_eq!(animate_frame(dummy_frames, 0, 1), "A");
    assert_eq!(animate_frame(dummy_frames, 1, 1), "B");
    assert_eq!(animate_frame(dummy_frames, 2, 1), "C");
    assert_eq!(animate_frame(dummy_frames, 3, 1), "A");
    assert_eq!(animate_frame(dummy_frames, 4, 1), "B");
}

#[test]
fn test_animate_frame_speed_divider() {
    let dummy_frames = &["Frame1", "Frame2"];
    
    // With speed_divider = 3, each frame should persist for 3 ticks
    assert_eq!(animate_frame(dummy_frames, 0, 3), "Frame1");
    assert_eq!(animate_frame(dummy_frames, 1, 3), "Frame1");
    assert_eq!(animate_frame(dummy_frames, 2, 3), "Frame1");
    assert_eq!(animate_frame(dummy_frames, 3, 3), "Frame2");
    assert_eq!(animate_frame(dummy_frames, 4, 3), "Frame2");
    assert_eq!(animate_frame(dummy_frames, 5, 3), "Frame2");
    assert_eq!(animate_frame(dummy_frames, 6, 3), "Frame1"); // Loops back
}

#[test]
fn test_animate_frame_empty_safeguard() {
    let empty_frames: &[&str] = &[];
    assert_eq!(animate_frame(empty_frames, 10, 1), "");
}

#[test]
fn test_effects_constants_exist() {
    // Asserting that we have common effects available
    assert!(!effects::ELECTRIC_SPARKS.is_empty());
    assert!(!effects::ENTITY_ZAPPED.is_empty());
}
