use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use genesis_tui::core::engine::controller::{EngineCommand, EngineController};
use genesis_tui::core::engine::Engine;
use genesis_tui::core::entity::Entity;
use genesis_tui::core::genetics::Genome;
use genesis_tui::core::grid::{Grid, Position, Velocity};
use genesis_tui::ui::Renderer;
use rand::Rng;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use std::panic;

fn main() -> io::Result<()> {
    // 1. Initialize the Universal Ghost Reporter
    let _telemetry_guard = genesis_tui::utils::telemetry::init_telemetry();
    tracing::info!("Initializing Genesis TUI Engine and Window Mode...");

    // Setup an emergency panic hook to tear down the raw terminal mode
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture);
        original_hook(panic_info);
    }));
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    io::stdout().execute(crossterm::event::EnableMouseCapture)?;

    let mut engine = Engine::new(Grid::new(200, 100));
    let mut rng = rand::thread_rng();
    for i in 1..=50 {
        let mut ent = Entity::new(
            i,
            Position::new(rng.gen_range(5.0..195.0), rng.gen_range(5.0..95.0)),
            Genome::new_random(16),
        );
        ent.set_velocity(Velocity::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ));
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
            renderer.draw(f, &state, tick_rate)
        })?;

        if event::poll(std::time::Duration::from_millis(tick_rate))? {
            let ev = event::read()?;
            match ev {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => {
                        engine_handle.send_command(EngineCommand::Quit);
                        break;
                    }
                    KeyCode::Char('r') => engine_handle.send_command(EngineCommand::Reset),
                    KeyCode::Char('1') => engine_handle.send_command(EngineCommand::ToggleFenceTop),
                    KeyCode::Char('2') => {
                        engine_handle.send_command(EngineCommand::ToggleFenceBottom)
                    }
                    KeyCode::Char('3') => {
                        engine_handle.send_command(EngineCommand::ToggleFenceLeft)
                    }
                    KeyCode::Char('4') => {
                        engine_handle.send_command(EngineCommand::ToggleFenceRight)
                    }
                    KeyCode::Char('5') => engine_handle.send_command(EngineCommand::ToggleFenceAll),
                    KeyCode::Char('p') => {
                        let state = engine_handle.get_state();
                        engine_handle.send_command(EngineCommand::SpawnRadarPing(
                            state.grid().width() as f32 / 2.0,
                            state.grid().height() as f32 / 2.0,
                        ));
                    }
                    KeyCode::Char('+') => {
                        engine_handle.send_command(EngineCommand::AddEntities(50))
                    }
                    KeyCode::Char('-') => {
                        engine_handle.send_command(EngineCommand::RemoveEntities(50))
                    }
                    KeyCode::Char('m') => {
                        engine_handle.send_command(EngineCommand::RandomizeVisuals)
                    }
                    KeyCode::Char('c') => engine_handle.send_command(EngineCommand::ResetVisuals),
                    KeyCode::Char(' ') => engine_handle.send_command(EngineCommand::TogglePause),
                    KeyCode::Right => engine_handle.send_command(EngineCommand::StepForward),
                    KeyCode::Left => engine_handle.send_command(EngineCommand::StepBackward),
                    KeyCode::Up => {
                        tick_rate = tick_rate.saturating_sub(10).max(16);
                        engine_handle.send_command(EngineCommand::SetTickRate(tick_rate));
                    }
                    KeyCode::Down => {
                        tick_rate = tick_rate.saturating_add(10).min(500);
                        engine_handle.send_command(EngineCommand::SetTickRate(tick_rate));
                    }
                    _ => {}
                },
                Event::Mouse(mouse_event) => {
                    if mouse_event.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
                        let state = engine_handle.get_state();
                        let (tw, th) = crossterm::terminal::size().unwrap_or((150, 60));
                        let grid_pos =
                            genesis_tui::utils::projection::ViewportProjection::term_to_grid(
                                mouse_event.column,
                                mouse_event.row,
                                tw,
                                th,
                                state.grid().width() as f32,
                                state.grid().height() as f32,
                            );
                        engine_handle.send_command(EngineCommand::Click(grid_pos.x, grid_pos.y));
                    } else if mouse_event.kind
                        == event::MouseEventKind::Down(event::MouseButton::Right)
                    {
                        let state = engine_handle.get_state();
                        let (tw, th) = crossterm::terminal::size().unwrap_or((150, 60));
                        let grid_pos =
                            genesis_tui::utils::projection::ViewportProjection::term_to_grid(
                                mouse_event.column,
                                mouse_event.row,
                                tw,
                                th,
                                state.grid().width() as f32,
                                state.grid().height() as f32,
                            );
                        engine_handle
                            .send_command(EngineCommand::RightClick(grid_pos.x, grid_pos.y));
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    io::stdout().execute(crossterm::event::DisableMouseCapture)?;

    // Wait for graceful background shutdown
    let _ = engine_thread.join();
    Ok(())
}
