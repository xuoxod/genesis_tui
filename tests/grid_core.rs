use genesis_tui::core::grid::{Grid, Position};

#[test]
fn test_grid_initialization() {
    let grid = Grid::new(100, 50);
    assert_eq!(grid.width(), 100);
    assert_eq!(grid.height(), 50);
}

#[test]
fn test_coordinate_bounds() {
    let grid = Grid::new(10, 10);
    assert!(grid.in_bounds(&Position::new(5, 5)));
    assert!(grid.in_bounds(&Position::new(0, 0)));
    assert!(grid.in_bounds(&Position::new(9, 9)));
    assert!(!grid.in_bounds(&Position::new(10, 5)));
    assert!(!grid.in_bounds(&Position::new(5, 10)));
    assert!(!grid.in_bounds(&Position::new(10, 10)));
}
