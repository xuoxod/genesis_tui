// --------------------------------------------------------
// DECOUPLED GENERIC ASCII ANIMATION EMITTER
// --------------------------------------------------------

/// An elegant, robust Eulerian-tick-based ASCII frame allocator.
/// Computes exactly which `&str` frame should be displayed at a given chronological tick,
/// allowing variable speed looping and safe modulo boundary math.
pub fn animate_frame<'a>(frames: &'a [&'a str], tick: usize, speed_divider: usize) -> &'a str {
    if frames.is_empty() {
        return "";
    }
    let safe_divider = if speed_divider == 0 { 1 } else { speed_divider };
    let cycle_index = (tick / safe_divider) % frames.len();
    frames[cycle_index]
}
