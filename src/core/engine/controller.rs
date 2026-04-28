use crate::core::engine::Engine;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub enum EngineCommand {
    Pause,
    Play,
    TogglePause,
    StepForward,
    StepBackward,
    Reset,
    SetTickRate(u64),
    Quit,
}

pub struct EngineHandle {
    sender: mpsc::Sender<EngineCommand>,
    state: Arc<RwLock<Engine>>,
}

pub struct EngineController;

impl EngineController {
    pub fn spawn(engine: Engine) -> (EngineHandle, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel::<EngineCommand>();
        let state = Arc::new(RwLock::new(engine));
        let worker_state = Arc::clone(&state);

        let handle = EngineHandle {
            sender: tx,
            state: Arc::clone(&state),
        };

        let join_handle = thread::spawn(move || {
            let mut tick_rate = 16; 
            let mut running = true;

            while running {
                let iteration_start = Instant::now();
                let mut should_tick = false;

                // Process all pending commands
                while let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        EngineCommand::Pause => { worker_state.write().unwrap().pause(); }
                        EngineCommand::Play => { worker_state.write().unwrap().play(); }
                        EngineCommand::TogglePause => { worker_state.write().unwrap().toggle_pause(); }
                        EngineCommand::StepForward => { worker_state.write().unwrap().step_forward(); }
                        EngineCommand::StepBackward => { worker_state.write().unwrap().step_backward(); }
                        EngineCommand::Reset => { worker_state.write().unwrap().reset(); }
                        EngineCommand::SetTickRate(rate) => { tick_rate = rate; }
                        EngineCommand::Quit => { running = false; }
                    }
                }

                if !running {
                    break;
                }

                {
                    let engine_read = worker_state.read().unwrap();
                    if !engine_read.is_paused() {
                        should_tick = true;
                    }
                }

                if should_tick {
                    worker_state.write().unwrap().tick();
                }

                let elapsed = iteration_start.elapsed();
                let tick_dur = Duration::from_millis(tick_rate);
                if elapsed < tick_dur {
                    thread::sleep(tick_dur - elapsed);
                }
            }
        });

        (handle, join_handle)
    }
}

impl EngineHandle {
    pub fn send_command(&self, cmd: EngineCommand) {
        let _ = self.sender.send(cmd);
    }

    pub fn get_state(&self) -> std::sync::RwLockReadGuard<'_, Engine> {
        self.state.read().unwrap()
    }
}
