                        EngineCommand::SpawnRadarPing(x, y) => {
                            worker_state.write().unwrap().spawn_radar_ping(x, y);
                        }
