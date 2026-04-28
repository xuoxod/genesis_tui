use ratatui::widgets::{Block, Borders, canvas::Canvas};
use ratatui::Frame;
use crate::core::engine::Engine;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self { Self }

    pub fn draw(&self, f: &mut Frame, e: &Engine) {
        let size = f.area();
        let c = Canvas::default().block(Block::default().borders(Borders::ALL))
            .x_bounds([0.0, e.grid().width() as f64])
            .y_bounds([0.0, e.grid().height() as f64])
            .paint(|ctx| {
                for ent in e.entities() {
                    ctx.print(ent.position().x as f64, ent.position().y as f64, "█");
                }
            });
        f.render_widget(c, size);
    }
}
