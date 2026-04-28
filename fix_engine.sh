git checkout src/core/engine/mod.rs
sed -i '/pub mod controller;/a\pub mod phenomena;' src/core/engine/mod.rs
sed -i 's/use crate::core::entity::Entity;/use crate::core::entity::Entity;\nuse crate::core::phenomena::Singularity;\nuse crate::utils::physics::gravitational_pull;/g' src/core/engine/mod.rs
sed -i 's/fence: ElectricFence,/fence: ElectricFence,\n    singularities: Vec<Singularity>,/g' src/core/engine/mod.rs
sed -i 's/fence: ElectricFence::new(),/fence: ElectricFence::new(),\n            singularities: Vec::new(),/g' src/core/engine/mod.rs
sed -i '/pub fn entities(&self) -> &\[Entity\] {/i\    pub fn singularities(&self) -> &\[Singularity\] {\n        &self.singularities\n    }\n\n    pub fn spawn_singularity(&mut self, grid_x: f32, grid_y: f32) {\n        self.singularities.push(Singularity::new(Position::new(grid_x, grid_y), 50.0, self.tick_count as usize, 100));\n    }\n' src/core/engine/mod.rs

sed -i '/let right_on = self.fence.is_active(FenceSide::Right);/i\        self.singularities.retain(|s| s.is_active(t_count as usize));\n' src/core/engine/mod.rs

sed -i '/let mut zapped = false;/a\            for singularity in &self.singularities {\n                let pull = gravitational_pull(\&singularity.position, \&pos, singularity.mass, 2.0);\n                vel.x += pull.x;\n                vel.y += pull.y;\n            }\n            vel.x *= 0.98;\n            vel.y *= 0.98;' src/core/engine/mod.rs

