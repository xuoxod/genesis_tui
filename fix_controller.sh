git checkout src/core/engine/controller.rs
sed -i '/Click(f32, f32),/a\    RightClick(f32, f32),' src/core/engine/controller.rs
sed -i -e '/EngineCommand::Click(x, y) => {/i \                        EngineCommand::RightClick(x, y) => {\n                            worker_state.write().unwrap().spawn_singularity(x, y);\n                        }' src/core/engine/controller.rs
