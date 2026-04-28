sed -i 's/renderer.draw(f, &state)/renderer.draw(f, \&state, tick_rate)/g' src/main.rs
