use crate::core::grid::{Position, Velocity};
use crate::core::genetics::Genome;
use crate::constants::colors;
use crate::utils::gradient::generate_gradient;
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Entity {
    id: u64,
    position: Position,
    velocity: Velocity,
    genome: Genome,
    
    // Electrification state
    is_electrified: bool,
    electrification_start_tick: usize,
    spark_gradient: Vec<Vec3>,
}

impl Entity {
    pub fn new(id: u64, position: Position, genome: Genome) -> Self {
        // High voltage electric effect gradient
        let gradient_stops = vec![colors::YELLOW, colors::WHITE, colors::CYAN, colors::NAVY, colors::MAGENTA];
        let spark_gradient = generate_gradient(&gradient_stops, 15); // 15-tick fast cycle
        
        // We initialize with a dead velocity and let the engine inject force, or derive it from the genome.
        Self { 
            id, 
            position, 
            velocity: Velocity::ZERO, 
            genome, 
            is_electrified: false,
            electrification_start_tick: 0,
            spark_gradient,
        }
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn position(&self) -> &Position { &self.position }
    pub fn velocity(&self) -> &Velocity { &self.velocity }
    pub fn genome(&self) -> &Genome { &self.genome }
    pub fn is_electrified(&self) -> bool { self.is_electrified }
    
    pub fn set_position(&mut self, pos: Position) { self.position = pos; }
    pub fn set_velocity(&mut self, vel: Velocity) { self.velocity = vel; }

    pub fn mutate_genome(&mut self, rate: f64) {
        self.genome.mutate(rate);
    }
    
    pub fn electrify(&mut self, current_tick: usize) {
        self.is_electrified = true;
        self.electrification_start_tick = current_tick;
    }
    
    pub fn cure_electrification(&mut self) {
        self.is_electrified = false;
    }

    pub fn get_render_effect(&self, current_tick: usize) -> Option<(&'static str, Vec3)> {
        if !self.is_electrified {
            return None;
        }
        
        let elapsed = current_tick.saturating_sub(self.electrification_start_tick);
        let cycle_index = elapsed % self.spark_gradient.len();
        let color = self.spark_gradient[cycle_index];
        
        // A series of nerdy electrified chars that animate over time!
        let chars = crate::constants::effects::ENTITY_ZAPPED;
        let frame = crate::utils::fx::animate_frame(chars, elapsed, 2);
        
        Some((frame, color))
    }
}
