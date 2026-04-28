cat << 'CODE' > src/main.rs
use genesis_tui::core::engine::Engine;
use genesis_tui::core::engine::controller::{EngineController, EngineCommand};
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
        let mut ent = Entity::new(i, Position::new(rng.gen_range(5.0..195.0), rng.gen_range(5.0..95.0)), Genome::new_random(16));
        ent.set_velocity(Velocity::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)));
        engine.add_entity(ent);
    }
    
    // Decoupling via Channel Architecture
    let (engine_handle, engine_thread) = EngineController::spawn(engine);
    let renderer = Renderer::new();
    let mut tick_rate = 16; 

    loop {
        // UI Thread fetches state via read lock (100% decoupled from math)
        terminal.draw(|f| {
            let state = engine_handle.get_state();
            renderer.draw(f, &state)
        })?;

        if event::poll(std::time::Duration::from_millis(tick_rate))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        engine_handle.send_command(EngineCommand::Quit);
                        break;
                    },
                    KeyCode::Char('r') => engine_handle.send_command(EngineCommand::Reset),
                    KeyCode::Char(' ') => engine_handle.send_command(EngineCommand::TogglePause), 
                    KeyCode::Right => engine_handle.send_command(EngineCommand::StepForward),     
                    KeyCode::Left => engine_handle.send_command(EngineCommand::StepBackward),     
                    KeyCode::Up => { 
                        tick_rate = tick_rate.saturating_sub(10).max(16); 
                        engine_handle.send_command(EngineCommand::SetTickRate(tick_rate));
                    },
                    KeyCode::Down => { 
                        tick_rate = tick_rate.saturating_add(10).min(500); 
                        engine_handle.send_command(EngineCommand::SetTickRate(tick_rate));
                    },
                    _ => {}
                }
            }
        }
    }
    
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    
    // Wait for graceful background shutdown
    let _ = engine_thread.join();
    Ok(())
}
CODE
