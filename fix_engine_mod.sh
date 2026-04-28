cat << 'CODE' > src/core/engine/mod.rs
pub mod controller;

use crate::core::grid::{Grid, Position, Velocity};
use crate::core::entity::Entity;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Engine {
    grid: Grid,
    entities: Vec<Entity>,
    paused: bool,
    tick_count: u64,
    fossil_record: VecDeque<Vec<Entity>>, 
}

impl Engine {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            entities: Vec::new(),
            paused: false,
            tick_count: 0,
            fossil_record: VecDeque::new(),
        }
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

    pub fn entities(&self) -> &[Entity] {
        &self.entities
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
        let bounds = self.grid.bounds();

        for entity in &mut self.entities {
            let pos = entity.position().clone();
            let mut vel = entity.velocity().clone();

            if pos.x <= 0.0 {
                vel.x = vel.x.abs(); 
            } else if pos.x >= bounds.x as f32 {
                vel.x = -vel.x.abs();
            }

            if pos.y <= 0.0 {
                vel.y = vel.y.abs();
            } else if pos.y >= bounds.y as f32 {
                vel.y = -vel.y.abs();
            }
            entity.set_velocity(vel);
            entity.update();
            
            let constrain_pos = Position::new(
                entity.position().x.clamp(0.0, bounds.x as f32),
                entity.position().y.clamp(0.0, bounds.y as f32)
            );
            entity.set_position(constrain_pos);
        }
    }

    pub fn tick(&mut self) {
        if !self.paused {
            self.tick_internal();
        }
    }
}
CODE
