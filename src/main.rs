use genesis_tui::core::engine::Engine;
use genesis_tui::core::grid::{Grid, Position};
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;
use genesis_tui::ui::Renderer;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let grid = Grid::new(200, 100);
    let mut engine = Engine::new(grid);
    let adam = Entity::new(1, Position::new(100, 50), Genome::new_random(16));
    engine.add_entity(adam);

    let renderer = Renderer::new();

    loop {
        terminal.draw(|f| renderer.draw(f, &engine))?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') { break; }
            }
        }
        engine.tick();
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
