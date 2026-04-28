# User Guide & TUI Keybindings

Genesis TUI features a hyper-responsive, color-coded graphical dashboard built for rapid iteration and simulation analysis.

## Live Telemetry Dashboard
At the bottom of the active instance, you will find the Telemetry Footer:
`▶ RUNNING | TICKS: XXXXXX | ENTITIES: XXX | SPEED: XXms/T || [Q]uit ...`

- **Play State:** Visually tracks whether the simulation is rolling (`RUNNING`) or halted (`PAUSED`).
- **TICKS:** The total number of chronological physics calculations passed since `t=0`.
- **SPEED:** The background thread's integration-delay polling rate. Lower is faster.

## Interactive Keybindings
These keys asynchronously trigger events without halting UI frames:

| Key | Action | Description |
| :--- | :--- | :--- |
| `q` | **Exit** | Shuts down the background thread and releases control over `STDOUT`. |
| `Space` | **Pause / Play** | Toggles the active progression of Time. |
| `Up Arrow` | **Speed Up** | Accelerates time by reducing engine sleep latency. |
| `Down Arrow`| **Slow Down** | Decelerates time to allow micro-inspection of elastic collisions. |
| `Right Arrow`| **Step Frame** | Calculates one explicit tick into the future (Best used while paused). |
| `Left Arrow`| **Time Travel** | Scrubs backward through the `fossil_record`, restoring physical positions backward in time. |
| `r` | **Reset World** | Instantly zeroizes the Fossil Record, Entity map, and Tick count. |
