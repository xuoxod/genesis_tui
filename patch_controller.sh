sed -i '/SetTickRate(u64),/a\    ToggleFenceAll,\n    ToggleFenceTop,\n    ToggleFenceBottom,\n    ToggleFenceLeft,\n    ToggleFenceRight,' src/core/engine/controller.rs
cat << 'PATCH' > replace_match.rs
                        EngineCommand::ToggleFenceAll => {
                            let mut w = worker_state.write().unwrap();
                            if w.fence().is_active(crate::utils::fence::FenceSide::Top) {
                                w.fence_mut().turn_off_all();
                            } else {
                                w.fence_mut().turn_on_all();
                            }
                        }
                        EngineCommand::ToggleFenceTop => { 
                            let mut w = worker_state.write().unwrap();
                            let s = w.fence().is_active(crate::utils::fence::FenceSide::Top);
                            if s { w.fence_mut().turn_off(crate::utils::fence::FenceSide::Top); } else { w.fence_mut().turn_on(crate::utils::fence::FenceSide::Top); }
                        }
                        EngineCommand::ToggleFenceBottom => { 
                            let mut w = worker_state.write().unwrap();
                            let s = w.fence().is_active(crate::utils::fence::FenceSide::Bottom);
                            if s { w.fence_mut().turn_off(crate::utils::fence::FenceSide::Bottom); } else { w.fence_mut().turn_on(crate::utils::fence::FenceSide::Bottom); }
                        }
                        EngineCommand::ToggleFenceLeft => { 
                            let mut w = worker_state.write().unwrap();
                            let s = w.fence().is_active(crate::utils::fence::FenceSide::Left);
                            if s { w.fence_mut().turn_off(crate::utils::fence::FenceSide::Left); } else { w.fence_mut().turn_on(crate::utils::fence::FenceSide::Left); }
                        }
                        EngineCommand::ToggleFenceRight => { 
                            let mut w = worker_state.write().unwrap();
                            let s = w.fence().is_active(crate::utils::fence::FenceSide::Right);
                            if s { w.fence_mut().turn_off(crate::utils::fence::FenceSide::Right); } else { w.fence_mut().turn_on(crate::utils::fence::FenceSide::Right); }
                        }
PATCH
sed -i '/EngineCommand::Quit => { running = false; }/r replace_match.rs' src/core/engine/controller.rs
