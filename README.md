# Genesis-TUI 🧬 (Procedural Digital Terrarium)

A "Zero-Player" ecosystem simulator and procedural evolution engine running entirely in the terminal. Define starting parameters in a YAML file, execute the binary, and watch species evolve, consume resources, mutate, and die off in real-time.

## Features
* **The Grid:** A live, shifting topographical map using high-density block characters (`⡷`, `⣯`, `⢀`) and ANSI styling.
* **Genetic Algorithms:** Entities carry mutable DNA sequences. Apex survivors reproduce and drift genetically.
* **Extinction Events:** Real-time keystroke interactions to trigger ecosystem-wide shifts (e.g., `[M]` for Meteor Strike).
* **The Fossil Record:** Background SQLite engine logging DNA snapshots of epoch-surviving species.

## Architecture
* **Core Engine:** Rust + `tokio` for massively concurrent, multi-threaded autonomous entity processing without UI lockup.
* **UI:** Built with `ratatui` and `crossterm` for cinematic, flicker-free rendering.
* **Persistence:** `rusqlite` for the Fossil Record and `serde_yaml` for seed topology configurations.

## Project Layout
* `src/core/` - The cellular automata simulation and genetic logic engine.
* `src/ui/` - Terminal rendering, map drawing, and the interactive event loop.
* `src/db/` - SQLite bindings for saving long-term simulations.
* `src/utils/` - Constants, colors, and shared logic.

## Usage
\`\`\`bash
# Compile with pure performance optimizations
cargo build --release

# Run the terrarium
cargo run --release
\`\`\`
