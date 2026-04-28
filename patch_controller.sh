sed -i '/Click(f32, f32),/a\    RightClick(f32, f32),' src/core/engine/controller.rs

cat << 'PATCH' > replace_rightclick.rs
                        EngineCommand::RightClick(x, y) => { 
                            worker_state.write().unwrap().spawn_singularity(x, y);
                        }
PATCH
sed -i '/EngineCommand::Click(x, y) => {/r replace_rightclick.rs' src/core/engine/controller.rs
