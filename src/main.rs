use genesis_tui::core::engine::Engine;
use genesis_tui::core::grid::{Grid, Position, Velocity};
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;
use genesis_tui::ui::Renderer;

use rand::Rng;
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
    let mut rng = rand::thread_rng();
    for i in 1..=50 {
        let x = rng.gen_range(5.0..195.0);
        let y = rng.gen_range(5.0..95.0);
        let vx = rng.gen_range(-1.0..1.0);
        let vy = rng.gen_range(-1.0..1.0);
        let mut ent = Entity::new(i, Position::new(x, y), Genome::new_random(16));
        ent.set_velocity(Velocity::new(vx, vy));
        engine.add_entity(ent);
    }

    let renderer = Renderer::new();
    let mut tick_rate = 16; 

    loop {
        terminal.draw(|f| renderer.draw(f, &engine))?;
        if event::poll(std::time::Duration::from_millis(tick_rate))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => engine.reset(),
                    KeyCode::Char(' ') => engine.toggle_pause(), 
                    KeyCode::Right => engine.step_forward(),     
                    KeyCode::Left => engine.step_backward(),     
                    KeyCode::Up => { tick_rate = tick_rate.saturating_sub(10).max(16); },
                    KeyCode::Down => { tick_rate = tick_rate.saturating_add(10).min(500); },
                    _ => {}
                }
            }
        } else {
            engine.tick();
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
