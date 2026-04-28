use crate::core::grid::{Position, Velocity};

/// Calculates the gravitational pull vector to apply to an entity.
/// Uses a simplified inverse-square law with a distance clamp to prevent infinite velocity at the center.
pub fn gravitational_pull(
    source: &Position,
    target: &Position,
    mass: f32,
    max_force: f32,
) -> Velocity {
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

/// Checks if a point lies within a specific thickness of an expanding ring.
/// Used for echolocation/radar sweeps.
pub fn ring_intersection(point: &Position, center: &Position, radius: f32, thickness: f32) -> bool {
    let dx = point.x - center.x;
    let dy = point.y - center.y;
    let dist = (dx * dx + dy * dy).sqrt();
    (dist - radius).abs() <= thickness
}

/// Boids: Cohesion - Steer towards the center of mass of local flockmates
pub fn boids_cohesion(target: &Position, neighbors: &[(Position, Velocity)]) -> Velocity {
    if neighbors.is_empty() {
        return Velocity::new(0.0, 0.0);
    }

    let mut center_x = 0.0;
    let mut center_y = 0.0;
    for (pos, _) in neighbors {
        center_x += pos.x;
        center_y += pos.y;
    }
    center_x /= neighbors.len() as f32;
    center_y /= neighbors.len() as f32;

    // Normalize direction towards center
    let dx = center_x - target.x;
    let dy = center_y - target.y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist > 0.0001 {
        Velocity::new(dx / dist, dy / dist)
    } else {
        Velocity::new(0.0, 0.0)
    }
}

/// Boids: Alignment - Steer towards the average heading of local flockmates
pub fn boids_alignment(target_vel: &Velocity, neighbors: &[(Position, Velocity)]) -> Velocity {
    if neighbors.is_empty() {
        return Velocity::new(0.0, 0.0);
    }

    let mut avg_vel_x = 0.0;
    let mut avg_vel_y = 0.0;
    for (_, vel) in neighbors {
        avg_vel_x += vel.x;
        avg_vel_y += vel.y;
    }
    avg_vel_x /= neighbors.len() as f32;
    avg_vel_y /= neighbors.len() as f32;

    let dx = avg_vel_x - target_vel.x;
    let dy = avg_vel_y - target_vel.y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist > 0.0001 {
        Velocity::new(dx / dist, dy / dist)
    } else {
        Velocity::new(0.0, 0.0)
    }
}

/// Boids: Separation - Steer to avoid crowding local flockmates
pub fn boids_separation(
    target: &Position,
    neighbors: &[(Position, Velocity)],
    min_distance: f32,
) -> Velocity {
    let mut steer_x = 0.0;
    let mut steer_y = 0.0;
    let mut count = 0;

    for (pos, _) in neighbors {
        let dx = target.x - pos.x;
        let dy = target.y - pos.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.0001 && dist < min_distance {
            // Further away = less force (inverse distance)
            steer_x += dx / dist;
            steer_y += dy / dist;
            count += 1;
        }
    }

    if count > 0 {
        steer_x /= count as f32;
        steer_y /= count as f32;

        let mag = (steer_x * steer_x + steer_y * steer_y).sqrt();
        if mag > 0.0001 {
            return Velocity::new(steer_x / mag, steer_y / mag);
        }
    }

    Velocity::new(0.0, 0.0)
}
