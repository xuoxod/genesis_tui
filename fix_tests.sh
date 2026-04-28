sed -i 's/Position::new(100, 50)/Position::new(100.0, 50.0)/g' src/main.rs
sed -i 's/Position::new(10, 20)/Position::new(10.0, 20.0)/g' tests/entity_core.rs
sed -i 's/Position::new(0, 0)/Position::new(0.0, 0.0)/g' tests/entity_core.rs
sed -i 's/Position::new(50, 50)/Position::new(50.0, 50.0)/g' tests/engine_core.rs
sed -i 's/Position::new(0,0)/Position::new(0.0, 0.0)/g' tests/engine_core.rs

sed -i 's/Position::new(5, 5)/Position::new(5.0, 5.0)/g' tests/grid_core.rs
sed -i 's/Position::new(0, 0)/Position::new(0.0, 0.0)/g' tests/grid_core.rs
sed -i 's/Position::new(9, 9)/Position::new(9.0, 9.0)/g' tests/grid_core.rs
sed -i 's/Position::new(10, 5)/Position::new(10.0, 5.0)/g' tests/grid_core.rs
sed -i 's/Position::new(5, 10)/Position::new(5.0, 10.0)/g' tests/grid_core.rs
sed -i 's/Position::new(10, 10)/Position::new(10.0, 10.0)/g' tests/grid_core.rs
