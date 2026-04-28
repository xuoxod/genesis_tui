cat << 'TICKINT' > update_tick_internal.rs
        // Clean up expired singularities
        self.singularities.retain(|s| s.is_active(t_count as usize));

        for entity in &mut self.entities {
            let pos = entity.position().clone();
            let mut vel = entity.velocity().clone();
            let mut zapped = false;
            
            // Apply physics from active phenomena (Singularities)
            for singularity in &self.singularities {
                let pull = gravitational_pull(&singularity.position, &pos, singularity.mass, 2.0); // max force capped
                vel.x += pull.x;
                vel.y += pull.y;
            }
            
            // Kinetic drag/damping to prevent infinite acceleration loops
            vel.x *= 0.95;
            vel.y *= 0.95;
TICKINT

sed -i -e '/for entity in \&mut self.entities {/r update_tick_internal.rs' -e '/for entity in \&mut self.entities {/d' src/core/engine/mod.rs
# wait, my update_tick_internal.rs doesn't delete the `let pos =...` lines which I'm redefining...
# Let's just do this explicitly with grep/sed/awk or git checkout
git checkout src/core/engine/mod.rs
sed -i '/pub mod controller;/a\pub mod phenomena;' src/core/engine/mod.rs
sed -i 's/use crate::core::entity::Entity;/use crate::core::entity::Entity;\nuse crate::core::phenomena::Singularity;\nuse crate::utils::physics::gravitational_pull;/g' src/core/engine/mod.rs
sed -i 's/fence: ElectricFence,/fence: ElectricFence,\n    singularities: Vec<Singularity>,/g' src/core/engine/mod.rs
sed -i 's/fence: ElectricFence::new(),/fence: ElectricFence::new(),\n            singularities: Vec::new(),/g' src/core/engine/mod.rs
sed -i '/pub fn entities(&self) -> &\[Entity\] {/i\    pub fn singularities(&self) -> &\[Singularity\] {\n        &self.singularities\n    }\n\n    pub fn spawn_singularity(&mut self, grid_x: f32, grid_y: f32) {\n        self.singularities.push(Singularity::new(Position::new(grid_x, grid_y), 50.0, self.tick_count as usize, 100));\n    }\n' src/core/engine/mod.rs

sed -i '/let right_on/i\        self.singularities.retain(|s| s.is_active(t_count as usize));\n' src/core/engine/mod.rs

sed -i '/let mut zapped = false;/a\            for singularity in &self.singularities {\n                let pull = gravitational_pull(\&singularity.position, \&pos, singularity.mass, 2.0);\n                vel.x += pull.x;\n                vel.y += pull.y;\n            }\n            vel.x *= 0.98;\n            vel.y *= 0.98;' src/core/engine/mod.rs

