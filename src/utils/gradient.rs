use glam::Vec3;

/// Core generic utility to linearly interpolate between exactly two colors.
pub fn lerp_color(start: Vec3, end: Vec3, t: f32) -> Vec3 {
    start.lerp(end, t.clamp(0.0, 1.0))
}

/// Generates a smooth mathematical array of colors spanning multiple geometric waypoints.
/// Example: `generate_gradient(&[RED, YELLOW, GREEN], 100)`
/// Returns a `Vec<Vec3>` of exactly 100 color frames drifting smoothly from Red -> Yellow -> Green.
/// Completely decoupled from the Engine; can be called by UI, genetics math, or tracers at will.
pub fn generate_gradient(waypoints: &[Vec3], steps: usize) -> Vec<Vec3> {
    if waypoints.is_empty() {
        return vec![];
    }
    if waypoints.len() == 1 {
        return vec![waypoints[0]; steps];
    }
    if steps == 0 {
        return vec![];
    }
    if steps == 1 {
        return vec![waypoints[0]];
    }

    let mut gradient = Vec::with_capacity(steps);
    let segments = waypoints.len() - 1;

    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32; // 0.0 to 1.0 across the entire gradient life

        // Map global `t` to the specific waypoint segment it currently rests inside
        let scaled_t = t * segments as f32;
        let segment_index = (scaled_t.floor() as usize).min(segments - 1);
        let segment_t = scaled_t - segment_index as f32;

        let start = waypoints[segment_index];
        let end = waypoints[segment_index + 1];

        gradient.push(lerp_color(start, end, segment_t));
    }

    gradient
}
