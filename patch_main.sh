cat << 'PATCH' > replace_main.rs
                    KeyCode::Char('1') => engine_handle.send_command(EngineCommand::ToggleFenceTop),
                    KeyCode::Char('2') => engine_handle.send_command(EngineCommand::ToggleFenceBottom),
                    KeyCode::Char('3') => engine_handle.send_command(EngineCommand::ToggleFenceLeft),
                    KeyCode::Char('4') => engine_handle.send_command(EngineCommand::ToggleFenceRight),
                    KeyCode::Char('5') => engine_handle.send_command(EngineCommand::ToggleFenceAll),
PATCH
sed -i '/KeyCode::Char('\''r'\'') => engine_handle.send_command(EngineCommand::Reset),/r replace_main.rs' src/main.rs
