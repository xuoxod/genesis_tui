use crate::core::grid::{Position, Velocity};

/// Calculates the gravitational pull vector to apply to an entity.
/// Uses a simplified inverse-square law with a distance clamp to prevent infinite velocity at the center.
pub fn gravitational_pull(source: &Position, target: &Position, mass: f32, max_force: f32) -> Velocity {
    let dx = source.x - target.x;
    let dy = source.y - target.y;
    let dist_sq = dx * dx + dy * dy;

    if dist_sq < 1.0 {
        return Velocity::new(0.0, 0.0); // Inside the event horizon, avoid division by zero
    }

    let force = (mass / dist_sq).min(max_force);
    let dist = dist_sq.sqrt();
    
    // Normalize and scale by force
    let nx = dx / dist;
    let ny = dy / dist;

    Velocity::new(nx * force, ny * force)
}
