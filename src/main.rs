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

    let mut engine = Engine::new(Grid::new(200, 100));
    engine.add_entity(Entity::new(1, Position::new(100, 50), Genome::new_random(16)));

    let renderer = Renderer::new();
    let mut tick_rate = 16; 

    // The TUI Loop
    loop {
        terminal.draw(|f| renderer.draw(f, &engine))?;
        
        // Wait for an event up to exactly our set Tick Rate limit
        if event::poll(std::time::Duration::from_millis(tick_rate))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => engine.reset(),
                    KeyCode::Char(' ') => engine.toggle_pause(), // Toggle Pause
                    KeyCode::Right => engine.step_forward(),     // Tick Forward (Slow Mo)
                    KeyCode::Left => engine.step_backward(),     // Time Traversal Reverse
                    KeyCode::Up => { tick_rate = tick_rate.saturating_sub(10).max(16); },
                    KeyCode::Down => { tick_rate = tick_rate.saturating_add(10).min(500); },
                    _ => {}
                }
            }
        } else {
            // Loop timed out normally, time to process a natural game tick
            engine.tick();
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
