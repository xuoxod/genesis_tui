cat << 'CODE' > src/ui/renderer.rs
use ratatui::{
    backend::Backend,
    widgets::{Block, Borders, canvas::Canvas},
    Frame,
};
use crate::core::engine::Engine;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self { Self }

    pub fn draw(&self, f: &mut Frame, engine: &Engine) {
        let size = f.area();
        let canvas = Canvas::default()
            .block(Block::default().title(" GENESIS ").borders(Borders::ALL))
            .x_bounds([0.0, engine.grid().width() as f64])
            .y_bounds([0.0, engine.grid().height() as f64])
            .paint(|ctx| {
                for e in engine.entities() {
                    ctx.print(e.position().x as f64, e.position().y as f64, "█");
                }
            });
        f.render_widget(canvas, size);
    }
}
CODE
