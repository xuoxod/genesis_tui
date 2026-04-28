use glam::Vec3;

/// Single Source of Truth for continuous RGB color states and geometric interpolation.
#[derive(Clone, Debug, PartialEq)]
pub struct RgbLerp {
    pub current: Vec3,
    pub targets: std::collections::VecDeque<Vec3>,
    pub speed: f32, // How much of the distance to cover per tick (0.0 to 1.0)
}

impl Default for RgbLerp {
    fn default() -> Self {
        Self::new(Vec3::new(255.0, 255.0, 255.0), 0.05)
    }
}

impl RgbLerp {
    /// Initialize with a starting color (R, G, B) and a transition speed (e.g., 0.05 for 5% per tick)
    pub fn new(start: Vec3, speed: f32) -> Self {
        Self {
            current: start,
            targets: std::collections::VecDeque::new(),
            speed: speed.clamp(0.001, 1.0),
        }
    }

    /// Queue a color transition dynamically
    pub fn push_target(&mut self, target: Vec3) {
        self.targets.push_back(target);
    }

    /// Progressively interpolates the current color towards the active target.
    /// Should be called during the Engine's tick loop.
    pub fn step(&mut self) {
        if let Some(target) = self.targets.front() {
            // Linear Interpolation (Lerp) algorithm
            self.current = self.current.lerp(*target, self.speed);

            // If we are chromatically "close enough" to the target, snap to it and pop the queue
            if self.current.distance(*target) < 1.0 {
                self.current = *target;
                self.targets.pop_front();
            }
        }
    }

    /// Output pure `u8` bytes for the Ratatui renderer
    pub fn as_bytes(&self) -> (u8, u8, u8) {
        (
            self.current.x.clamp(0.0, 255.0) as u8,
            self.current.y.clamp(0.0, 255.0) as u8,
            self.current.z.clamp(0.0, 255.0) as u8,
        )
    }
}
