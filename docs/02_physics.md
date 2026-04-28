# Physics & Kinematics Engine

Unlike traditional cellular automata (like Conway's Game of Life) that rely on discrete Integer (`usize`) grid coordinates, Genesis utilizes a continuous float-space architecture.

## Vector Mathematics (`glam`)
Using the industry-standard `glam` library (frequently utilized in Rust WebGPU/Gamedev engines like Bevy), all Entities are tracked via geometric Vectors (`Vec2::new(x, y)`).

### Eulerian Integration
During every `Engine::tick()`, the backend executes:
```rust
position += velocity;
```

### Perfectly Elastic Boundary Collisions
To contain organisms within the `Grid` bounding box, the simulation implements absolute momentum reflection. If an entity attempts to violate the `x = 0.0` or `x = grid.width` boundaries, its velocity vector is instantly inverted while preserving its momentum coefficient:
```rust
if pos.x <= 0.0 {
    vel.x = vel.x.abs(); 
} else if pos.x >= bounds_x {
    vel.x = -vel.x.abs();
}
```
This forces the programmatic entities to bounce identically to real-world billiard balls.
