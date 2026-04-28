pub mod controller;

use crate::core::entity::Entity;
use crate::core::grid::{Grid, Position, Velocity};
use crate::core::phenomena::{RadarPing, Singularity};
use crate::utils::fence::{ElectricFence, FenceSide};
use crate::utils::physics::{gravitational_pull, ring_intersection};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Engine {
    grid: Grid,
    entities: Vec<Entity>,
    paused: bool,
    tick_count: u64,
    fossil_record: VecDeque<Vec<Entity>>,
    fence: ElectricFence,
    singularities: Vec<Singularity>,
    radar_pings: Vec<RadarPing>,
}

impl Engine {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            entities: Vec::new(),
            paused: false,
            tick_count: 0,
            fossil_record: VecDeque::new(),
            fence: ElectricFence::new(),
            singularities: Vec::new(),
            radar_pings: Vec::new(),
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn fence(&self) -> &ElectricFence {
        &self.fence
    }

    pub fn fence_mut(&mut self) -> &mut ElectricFence {
        &mut self.fence
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn play(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn radar_pings(&self) -> &[RadarPing] {
        &self.radar_pings
    }

    pub fn spawn_radar_ping(&mut self, grid_x: f32, grid_y: f32) {
        self.radar_pings.push(RadarPing::new(
            Position::new(grid_x, grid_y),
            self.tick_count as usize,
            2.0,
            80.0,
        ));
    }

    pub fn singularities(&self) -> &[Singularity] {
        &self.singularities
    }

    pub fn spawn_singularity(&mut self, grid_x: f32, grid_y: f32) {
        self.singularities.push(Singularity::new(
            Position::new(grid_x, grid_y),
            50.0,
            self.tick_count as usize,
            100,
        ));
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn reset(&mut self) {
        self.entities.clear();
        self.tick_count = 0;
        self.fossil_record.clear();
        self.paused = false;
    }

    pub fn step_forward(&mut self) {
        self.tick_internal();
    }

    pub fn step_backward(&mut self) {
        self.paused = true;
        if let Some(previous_entities) = self.fossil_record.pop_back() {
            self.entities = previous_entities;
            self.tick_count = self.tick_count.saturating_sub(1);
        }
    }

    fn save_snapshot(&mut self) {
        self.fossil_record.push_back(self.entities.clone());
        if self.fossil_record.len() > 1000 {
            self.fossil_record.pop_front();
        }
    }

    fn tick_internal(&mut self) {
        self.save_snapshot();
        self.tick_count += 1;
        let bounds_x = self.grid.width() as f32;
        let bounds_y = self.grid.height() as f32;
        let t_count = self.tick_count;
        self.singularities.retain(|s| s.is_active(t_count as usize));
        self.radar_pings.retain(|p| p.is_active(t_count as usize));

        let right_on = self.fence.is_active(FenceSide::Right);
        let left_on = self.fence.is_active(FenceSide::Left);
        let top_on = self.fence.is_active(FenceSide::Top);
        let bot_on = self.fence.is_active(FenceSide::Bottom);

        let mut flock_data: Vec<(crate::core::grid::Position, crate::core::grid::Velocity)> =
            Vec::with_capacity(self.entities.len());
        for e_immut in &self.entities {
            flock_data.push((e_immut.position().clone(), e_immut.velocity().clone()));
        }
        for (i, entity) in self.entities.iter_mut().enumerate() {
            let pos = entity.position().clone();
            let mut vel = entity.velocity().clone();
            let mut zapped = false;
            for singularity in &self.singularities {
                for ping in &self.radar_pings {
                    let radius = ping.current_radius(t_count as usize);
                    if ring_intersection(&pos, &ping.position, radius, 2.0) {
                        // 2.0 thickness tolerance
                        entity.ping(t_count as usize);
                    }
                }
                let pull = crate::utils::physics::gravitational_pull(
                    &singularity.position,
                    &pos,
                    singularity.mass,
                    2.0,
                );
                vel.x += pull.x;
                vel.y += pull.y;
            }
            // Boids logic
            let mut neighbors = Vec::new();
            for (j, (n_pos, n_vel)) in flock_data.iter().enumerate() {
                if i != j {
                    let dist = ((pos.x - n_pos.x).powi(2) + (pos.y - n_pos.y).powi(2)).sqrt();
                    if dist < 10.0 {
                        // view radius
                        neighbors.push((n_pos.clone(), n_vel.clone()));
                    }
                }
            }
            let coh = crate::utils::physics::boids_cohesion(&pos, &neighbors);
            let ali = crate::utils::physics::boids_alignment(&vel, &neighbors);
            let sep = crate::utils::physics::boids_separation(&pos, &neighbors, 4.0);

            let boid_weight = 0.1;
            vel.x += (coh.x + ali.x + sep.x * 1.5) * boid_weight;
            vel.y += (coh.y + ali.y + sep.y * 1.5) * boid_weight;

            // Boids logic
            let mut neighbors = Vec::new();
            for (j, (n_pos, n_vel)) in flock_data.iter().enumerate() {
                if i != j {
                    let dist = ((pos.x - n_pos.x).powi(2) + (pos.y - n_pos.y).powi(2)).sqrt();
                    if dist < 10.0 {
                        // view radius
                        neighbors.push((n_pos.clone(), n_vel.clone()));
                    }
                }
            }
            let coh = crate::utils::physics::boids_cohesion(&pos, &neighbors);
            let ali = crate::utils::physics::boids_alignment(&vel, &neighbors);
            let sep = crate::utils::physics::boids_separation(&pos, &neighbors, 4.0);

            let boid_weight = 0.1;
            vel.x += (coh.x + ali.x + sep.x * 1.5) * boid_weight;
            vel.y += (coh.y + ali.y + sep.y * 1.5) * boid_weight;

            vel.x *= 0.98;
            vel.y *= 0.98;

            if pos.x <= 0.0 {
                vel.x = vel.x.abs();
                if left_on {
                    zapped = true;
                }
            } else if pos.x >= bounds_x {
                vel.x = -vel.x.abs();
                if right_on {
                    zapped = true;
                }
            }

            if pos.y <= 0.0 {
                vel.y = vel.y.abs();
                if top_on {
                    zapped = true;
                }
            } else if pos.y >= bounds_y {
                vel.y = -vel.y.abs();
                if bot_on {
                    zapped = true;
                }
            }
            entity.set_velocity(vel);

            if zapped {
                entity.electrify(t_count as usize);
            }

            let constrain_pos = Position::new(
                (pos.x + vel.x).clamp(0.0, bounds_x),
                (pos.y + vel.y).clamp(0.0, bounds_y),
            );
            entity.set_position(constrain_pos);
        }
    }

    pub fn handle_click(&mut self, grid_x: f32, grid_y: f32) {
        let t_count = self.tick_count as usize;
        let mut flock_data: Vec<(crate::core::grid::Position, crate::core::grid::Velocity)> =
            Vec::with_capacity(self.entities.len());
        for e_immut in &self.entities {
            flock_data.push((e_immut.position().clone(), e_immut.velocity().clone()));
        }
        for (i, entity) in self.entities.iter_mut().enumerate() {
            let dx = entity.position().x - grid_x;
            let dy = entity.position().y - grid_y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq <= 25.0 {
                entity.interact(t_count);
                entity.cure_electrification();
                let mut vel = entity.velocity().clone();
                vel.x += if dx > 0.0 { 1.5 } else { -1.5 };
                vel.y += if dy > 0.0 { 1.5 } else { -1.5 };
                entity.set_velocity(vel);
            }
        }
    }

    pub fn tick(&mut self) {
        if !self.paused {
            self.tick_internal();
        }
    }
}
