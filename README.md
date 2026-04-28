# Genesis TUI

A terminal-based procedural digital terrarium simulator built with Rust. Designed using strict TDD (Test-Driven Development), clean architecture (Separation of Concerns), and a decoupled background engine.

## Architecture

- **Physics Simulation:** Employs `f32` vectors via `glam` for professional-grade Newtonian kinematics and continuous spatial tracking. Includes perfectly elastic boundary collisions.
- **Concurrency (MPSC):** The simulation engine operates entirely in an isolated OS thread. Your keystrokes send non-blocking `mpsc` channel commands to the simulation, guaranteeing a flawless 60 FPS TUI rendering layer regardless of background math intensity.
- **The Fossil Record:** A rotating snapshot memory buffer saves chronological states, enabling flawless backwards time-travel.

## Controls

Because the UI layer is 100% decoupled from the simulation thread, these inputs are processed instantly over the active MPSC channel:

| Key | Action | Description |
| :--- | :--- | :--- |
| `q` | **Quit** | Gracefully tears down the background physics thread and exits. |
| `Spacebar` | **Pause / Play** | Freezes or unfreezes time. |
| `Up Arrow` | **Fast Forward** | Speeds up the simulation by decreasing the tick delay (down to ~60 FPS). |
| `Down Arrow` | **Slow Motion** | Slows down the simulation by increasing the tick delay (up to 500ms/tick). |
| `Right Arrow`| **Step Forward**| Explicitly calculates and advances exactly 1 frame forward. Best used while paused. |
| `Left Arrow` | **Time Travel (Rewind)**| Scrubs time backward! Steps incrementally backward into the local fossil record buffer and restores previous entity kinematics/positions. |
| `r` | **Reset** | Wipes the entire grid, clearing the fossil record and emptying all entities. |

## Quick Start

Ensure you have Rust and Cargo installed, then execute:

```bash
cargo run
```
# genesis_tui
