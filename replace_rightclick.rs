                        EngineCommand::RightClick(x, y) => { 
                            worker_state.write().unwrap().spawn_singularity(x, y);
                        }
