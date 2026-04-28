use genesis_tui::core::engine::Engine;
use genesis_tui::core::grid::Grid;
use genesis_tui::core::engine::controller::{EngineController, EngineCommand};
use std::time::Duration;

#[test]
fn test_engine_threading_decoupling() {
    let engine = Engine::new(Grid::new(100, 100));
    let (mut handle, join_handle) = EngineController::spawn(engine);

    // Initial state
    assert_eq!(handle.get_state().is_paused(), false);

    // Send Pause command
    handle.send_command(EngineCommand::Pause);
    
    // Give thread a moment to process
    std::thread::sleep(Duration::from_millis(50));
    
    // Verify it paused
    assert_eq!(handle.get_state().is_paused(), true);
    
    // Shut down the thread
    handle.send_command(EngineCommand::Quit);
    
    // Ensure thread exits cleanly
    assert!(join_handle.join().is_ok());
}
