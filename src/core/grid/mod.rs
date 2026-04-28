#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position { pub x: usize, pub y: usize }

impl Position {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y } }
}

#[derive(Debug, Clone)]
pub struct Grid { width: usize, height: usize }

impl Grid {
    pub fn new(width: usize, height: usize) -> Self { Self { width, height } }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}
