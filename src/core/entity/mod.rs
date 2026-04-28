use crate::constants::colors;
use crate::core::genetics::Genome;
use crate::core::grid::{Position, Velocity};
use crate::core::visuals::Shape;
use crate::utils::gradient::generate_gradient;
use glam::Vec3;
use std::collections::VecDeque;

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

    // Mouse Interaction state
    shape: Shape,
    base_color: Vec3,
    custom_gradient: Option<Vec<Vec3>>,
    is_interacting: bool,
    interaction_start_tick: usize,
    shimmer_gradient: Vec<Vec3>,
    pinged_start_tick: usize,
    trail: VecDeque<Position>,
}

impl Entity {
    pub fn new(id: u64, position: Position, genome: Genome) -> Self {
        // High voltage electric effect gradient
        let gradient_stops = vec![
            colors::YELLOW,
            colors::WHITE,
            colors::CYAN,
            colors::NAVY,
            colors::MAGENTA,
        ];
        let spark_gradient = generate_gradient(&gradient_stops, 15); // 15-tick fast cycle

        let shimmer_stops = vec![colors::MAGENTA, colors::CYAN, colors::MAGENTA];
        let shimmer_gradient = generate_gradient(&shimmer_stops, 20); // 20-tick pulse

        // We initialize with a dead velocity and let the engine inject force, or derive it from the genome.
        Self {
            id,
            position,
            velocity: Velocity::ZERO,
            genome,
            shape: Shape::Dot,
            base_color: colors::WHITE,
            custom_gradient: None,
            is_electrified: false,
            electrification_start_tick: 0,
            spark_gradient,
            is_interacting: false,
            interaction_start_tick: 0,
            shimmer_gradient,
            pinged_start_tick: 0,
            trail: VecDeque::with_capacity(10),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn velocity(&self) -> &Velocity {
        &self.velocity
    }
    pub fn genome(&self) -> &Genome {
        &self.genome
    }

    // Generic Decoupled Visual Methods
    pub fn shape(&self) -> Shape {
        self.shape
    }
    pub fn shape_char(&self) -> &'static str {
        self.shape.as_str()
    }
    pub fn base_color(&self) -> Vec3 {
        self.base_color
    }
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }
    pub fn set_base_color(&mut self, color: Vec3) {
        self.base_color = color;
    }
    pub fn apply_custom_gradient(&mut self, gradient: Vec<Vec3>) {
        self.custom_gradient = Some(gradient);
    }
    pub fn has_custom_gradient(&self) -> bool {
        self.custom_gradient.is_some()
    }
    pub fn custom_gradient(&self) -> Option<&Vec<Vec3>> {
        self.custom_gradient.as_ref()
    }
    pub fn reset_visuals(&mut self) {
        self.shape = Shape::Dot;
        self.base_color = crate::constants::colors::WHITE;
        self.custom_gradient = None;
    }

    pub fn trail(&self) -> &VecDeque<Position> {
        &self.trail
    }
    pub fn is_electrified(&self) -> bool {
        self.is_electrified
    }

    pub fn set_position(&mut self, pos: Position) {
        self.trail.push_back(self.position.clone());
        if self.trail.len() > 10 {
            self.trail.pop_front();
        }
        self.position = pos;
    }
    pub fn set_velocity(&mut self, vel: Velocity) {
        self.velocity = vel;
    }

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

    pub fn interact(&mut self, current_tick: usize) {
        self.is_interacting = true;
        self.interaction_start_tick = current_tick;
    }

    pub fn ping(&mut self, current_tick: usize) {
        self.pinged_start_tick = current_tick;
    }

    pub fn un_interact(&mut self) {
        self.is_interacting = false;
    }

    pub fn get_render_effect(&self, current_tick: usize) -> Option<(&'static str, Vec3)> {
        if self.is_electrified {
            // High priority ping flash
            let ping_elapsed = current_tick.saturating_sub(self.pinged_start_tick);
            if self.pinged_start_tick > 0 && ping_elapsed <= 2 {
                return Some(("X", Vec3::new(255.0, 255.0, 255.0)));
            }

            let elapsed = current_tick.saturating_sub(self.electrification_start_tick);
            let cycle_index = elapsed % self.spark_gradient.len();
            let color = self.spark_gradient[cycle_index];
            let chars = crate::constants::effects::ENTITY_ZAPPED;
            let frame = crate::utils::fx::animate_frame(chars, elapsed, 2);
            return Some((frame, color));
        }

        if self.is_interacting {
            let elapsed = current_tick.saturating_sub(self.interaction_start_tick);
            let cycle_index = elapsed % self.shimmer_gradient.len();
            let color = self.shimmer_gradient[cycle_index];
            let chars = crate::constants::effects::MUTATION_SHIMMER;
            let frame = crate::utils::fx::animate_frame(chars, elapsed, 3);

            // Interaction effect decays automatically after 30 ticks
            if elapsed > 30 {
                return None; // or we could self-mutate, but let the renderer manage visual decay
            }

            return Some((frame, color));
        }

        None
    }
}
