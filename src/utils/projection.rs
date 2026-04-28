use glam::Vec2;

/// Utility for translating terminal characters (cols/rows) into our continuous Euclidean plane (f32).
pub struct ViewportProjection;

impl ViewportProjection {
    /// Maps a raw Terminal Click (col, row) to the Engine's physical Grid (x, y).
    pub fn term_to_grid(
        term_x: u16,
        term_y: u16,
        term_width: u16,
        term_height: u16,
        grid_width: f32,
        grid_height: f32,
    ) -> Vec2 {
        let x_ratio = (term_x as f32) / (term_width as f32).max(1.0);
        let y_ratio = (term_y as f32) / (term_height as f32).max(1.0);

        let mut sim_y = y_ratio * grid_height;
        // TUI Canvases are inverted on the Y axis (0,0 is bottom left for math, top-left for term)
        sim_y = grid_height - sim_y;

        Vec2::new(x_ratio * grid_width, sim_y)
    }
}
