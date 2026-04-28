cat << 'MOD' > src/core/grid/mod.rs
pub use glam::Vec2 as Position;
pub use glam::Vec2 as Velocity;

#[derive(Debug, Clone)]
pub struct Grid { width: usize, height: usize }

impl Grid {
    pub fn new(width: usize, height: usize) -> Self { Self { width, height } }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0.0 && pos.x < self.width as f32 &&
        pos.y >= 0.0 && pos.y < self.height as f32
    }
}
MOD

cat << 'MOD' > src/core/entity/mod.rs
use crate::core::grid::{Position, Velocity};
use crate::core::genetics::Genome;

#[derive(Debug, Clone)]
pub struct Entity {
    id: u64,
    position: Position,
    velocity: Velocity,
    genome: Genome,
}

impl Entity {
    pub fn new(id: u64, position: Position, genome: Genome) -> Self {
        // We initialize with a dead velocity and let the engine inject force, or derive it from the genome.
        Self { id, position, velocity: Velocity::ZERO, genome }
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn position(&self) -> &Position { &self.position }
    pub fn velocity(&self) -> &Velocity { &self.velocity }
    pub fn genome(&self) -> &Genome { &self.genome }
    
    pub fn set_position(&mut self, pos: Position) { self.position = pos; }
    pub fn set_velocity(&mut self, vel: Velocity) { self.velocity = vel; }

    pub fn mutate_genome(&mut self, rate: f64) {
        self.genome.mutate(rate);
    }
}
MOD

cat << 'MOD' > src/core/engine/mod.rs
use crate::core::grid::Grid;
use crate::core::entity::Entity;

pub struct Engine {
    grid: Grid,
    entities: Vec<Entity>,
    epoch: u64,
    is_paused: bool,
    fossil_record: Vec<(u64, Vec<Entity>)>,
}

impl Engine {
    pub fn new(grid: Grid) -> Self { 
        Self { grid, entities: Vec::new(), epoch: 0, is_paused: false, fossil_record: Vec::new() } 
    }
    
    pub fn epoch(&self) -> u64 { self.epoch }
    pub fn entities(&self) -> &[Entity] { &self.entities }
    pub fn grid(&self) -> &Grid { &self.grid }
    pub fn is_paused(&self) -> bool { self.is_paused }
    
    pub fn add_entity(&mut self, e: Entity) { self.entities.push(e); }
    pub fn toggle_pause(&mut self) { self.is_paused = !self.is_paused; }
    
    pub fn reset(&mut self) {
        self.epoch = 0;
        self.entities.clear();
        self.fossil_record.clear();
        self.is_paused = false;
    }

    pub fn tick(&mut self) {
        if !self.is_paused { self.step_forward(); }
    }

    pub fn step_forward(&mut self) {
        // Save state to fossil record
        if self.fossil_record.len() >= 1000 { self.fossil_record.remove(0); }
        self.fossil_record.push((self.epoch, self.entities.clone()));
        
        // 1. Advance Epoch
        self.epoch += 1;

        // 2. Physics & Kinematics Pass
        let w = self.grid.width() as f32;
        let h = self.grid.height() as f32;

        for entity in &mut self.entities {
            let mut pos = *entity.position();
            let mut vel = *entity.velocity();

            // Newtonian Kinematics
            pos += vel;

            // Elastic Boundary Collisions (Perfect Bounce)
            if pos.x < 0.0 {
                pos.x = 0.0;
                vel.x = vel.x.abs();
            } else if pos.x >= w {
                pos.x = w - 0.01;
                vel.x = -vel.x.abs();
            }

            if pos.y < 0.0 {
                pos.y = 0.0;
                vel.y = vel.y.abs();
            } else if pos.y >= h {
                pos.y = h - 0.01;
                vel.y = -vel.y.abs();
            }

            entity.set_position(pos);
            entity.set_velocity(vel);
        }
    }

    pub fn step_backward(&mut self) {
        if let Some((prev_epoch, prev_ent)) = self.fossil_record.pop() {
            self.epoch = prev_epoch;
            self.entities = prev_ent;
        }
    }
}
MOD
