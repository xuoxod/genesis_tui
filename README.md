# Genesis TUI Engine

[![Rust](https://img.shields.io/badge/rust-v1.85+-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Architecture](https://img.shields.io/badge/architecture-MPSC%20Decoupled-orange)](docs/01_architecture.md)

**Genesis TUI** is an enterprise-grade, procedural digital terrarium and genetic simulator built entirely in Rust. Running directly in your terminal, it leverages rigorous Newtonian kinematics, concurrent message-passing architecture, and a specialized "fossil record" buffer to deliver a 60 FPS, time-travel-capable simulation environment.

## Key Capabilities

- **Massively Decoupled Architecture:** The simulation math and TUI rendering operate on entirely different OS threads, synchronizing state purely via lock-free `Arc<RwLock>` reads and MPSC command channels.
- **Continuous `f32` Physics:** Grid-based mapping is abandoned in favor of continuous Euclidean space driven by `glam`. Entities possess high-precision vectors, executing perfectly elastic bounding-box collisions.
- **Chronological Time Travel:** The Engine maintains a sliding `fossil_record` buffer. Users can freeze reality and scrub backward through the mathematical history of the terrarium frame-by-frame.
- **Enterprise Telemetry:** Features a bottom-anchored, color-coded `ratatui` dashboard displaying live Ticks, Entity Counts, Engine Polling Rates, and interactive Play/Pause states.
- **Test-Driven Foundation:** Built strictly using TDD principles, heavily implementing Integration Tests before core concepts are ever rendered to the screen.

## Documentation Suite

We maintain a strict separation between high-level overviews and granular engineering decisions. Please refer to our documentation modules:

- [**01. Core Architecture & Threading**](docs/01_architecture.md) - Details on MPSC, State Locks, and the Engine Loop.
- [**02. Physics & Kinematics Engine**](docs/02_physics.md) - Explanation of `glam` integrations, clipping, and Eulerian math.
- [**03. User Guide & TUI Keybindings**](docs/03_user_guide.md) - Full breakdown of the console interface and interactive controls.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/xuoxod/genesis_tui.git
cd genesis_tui

# Run the localized engine
cargo run
```
