# Core Architecture & Threading

The Genesis TUI Engine rejects monolithic graphical loops in favor of a strictly decoupled, highly-concurrent Threading Architecture. It employs two distinct Operational Threads coordinated by standard library concurrency primitives.

## 1. The Simulation Thread (Engine Worker)
Spawned completely independently of the visual layer, the `EngineController` runs a non-blocking `while` loop that handles:
- **Kinematics:** Math integration (`Velocity` applied to `Position`).
- **Data Capture:** Taking snapshots of the current map state and pushing them into the Chronological `fossil_record` buffer.
- **Command Handling:** Ingesting asynchronous inputs via `mpsc` from the user.

## 2. The Render Thread (Main UI Loop)
Driven by `crossterm` and `ratatui`, the UI thread does **no physics math**. Its sole purpose is:
- **Snapshot Polling:** Obtaining a read-only lock (`RwLockReadGuard`) on the Engine state and mapping the floating-point entities to terminal UI Braille/Block characters.
- **Input Parsing:** Capturing keyboard strokes at 60 FPS and dispatching them down the `mpsc` channel as `EngineCommand` enums.

## State Isolation Paradigm
By isolating the Engine data behind an `Arc<RwLock<Engine>>`, the background thread can exclusively hold the write lock while integrating the next physics frame, while the UI thread simply "skips" to the latest available read lock state when drawing the 60 FPS frame map. 
