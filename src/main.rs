use genesis_tui::core::grid::{Grid, Position};
use genesis_tui::core::genetics::Genome;
use genesis_tui::core::entity::Entity;

fn main() {
    println!("[ GENESIS-TUI ] Bootstrap sequence initiated...");
    let world_grid = Grid::new(200, 100);
    println!("World grid initialized: {}x{}", world_grid.width(), world_grid.height());
    let _adam = Entity::new(1, Position::new(100, 50), Genome::new_random(16));
    println!("First core entity initialized matching constraints.");
}
