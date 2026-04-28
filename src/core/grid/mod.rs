pub use glam::Vec2 as Position;
pub use glam::Vec2 as Velocity;

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0.0 && pos.x < self.width as f32 && pos.y >= 0.0 && pos.y < self.height as f32
    }
}
