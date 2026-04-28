use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::EnvFilter;

/// Initializes the headless trace system for terminal UI projects.
/// It aggressively streams logs to a file (`logs/genesis.log`) without corrupting stdout.
pub fn init_telemetry() -> tracing_appender::non_blocking::WorkerGuard {
    // We send everything to a daily rolling log file in the `logs` directory
    let file_appender = rolling::never("logs", "genesis.log");

    // We make it non-blocking so tracing never slows down the 60FPS engine Math
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Set up a default dynamic filter (e.g., debug and above, or info)
    // You can override this using the RUST_LOG environment variable.
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    // Build the subscriber that routes data.
    // We disable ANSI color inside the file for clean regex parsing.
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking.with_max_level(Level::TRACE))
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .with_ansi(false) // No messy escape codes in the log file
        .init();

    tracing::info!("--- TELEMETRY SYSTEMS ONLINE ---");
    tracing::debug!("Non-blocking file appender attached to logs/genesis.log");

    // We must return the guard, assigning it in main keeps the background thread alive.
    guard
}
