# Entity Updates
# Add pinged_start_tick to Entity
sed -i 's/shimmer_gradient: Vec<Vec3>,/shimmer_gradient: Vec<Vec3>,\n    pinged_start_tick: usize,/g' src/core/entity/mod.rs
sed -i 's/shimmer_gradient,/shimmer_gradient,\n            pinged_start_tick: 0,/g' src/core/entity/mod.rs
sed -i '/pub fn un_interact(&mut self) {/i\    pub fn ping(&mut self, current_tick: usize) {\n        self.pinged_start_tick = current_tick;\n    }\n' src/core/entity/mod.rs

# Inside get_render_effect
cat << 'EFF' > patch_render_effect.rs
        // High priority ping flash
        let ping_elapsed = current_tick.saturating_sub(self.pinged_start_tick);
        if self.pinged_start_tick > 0 && ping_elapsed <= 2 {
            return Some(("X", Vec3::new(255.0, 255.0, 255.0)));
        }

EFF
sed -i -e '/if self.is_electrified {/r patch_render_effect.rs' src/core/entity/mod.rs

# Engine Updates
sed -i 's/use crate::core::phenomena::Singularity;/use crate::core::phenomena::{Singularity, RadarPing};/g' src/core/engine/mod.rs
sed -i 's/use crate::utils::physics::gravitational_pull;/use crate::utils::physics::{gravitational_pull, ring_intersection};/g' src/core/engine/mod.rs

sed -i 's/singularities: Vec<Singularity>,/singularities: Vec<Singularity>,\n    radar_pings: Vec<RadarPing>,/g' src/core/engine/mod.rs
sed -i 's/singularities: Vec::new(),/singularities: Vec::new(),\n            radar_pings: Vec::new(),/g' src/core/engine/mod.rs

sed -i '/pub fn singularities(&self) -> &\[Singularity\] {/i\    pub fn radar_pings(&self) -> &\[RadarPing\] {\n        &self.radar_pings\n    }\n\n    pub fn spawn_radar_ping(&mut self, grid_x: f32, grid_y: f32) {\n        self.radar_pings.push(RadarPing::new(Position::new(grid_x, grid_y), self.tick_count as usize, 2.0, 80.0));\n    }\n' src/core/engine/mod.rs

cat << 'PIN' > patch_ping_internal.rs
        self.radar_pings.retain(|p| p.is_active(t_count as usize));
PIN

sed -i -e '/self.singularities.retain/r patch_ping_internal.rs' src/core/engine/mod.rs

cat << 'PIN2' > patch_ping_ent.rs
            for ping in &self.radar_pings {
                let radius = ping.current_radius(t_count as usize);
                if ring_intersection(&pos, &ping.position, radius, 2.0) { // 2.0 thickness tolerance
                    entity.ping(t_count as usize);
                }
            }
PIN2
sed -i -e '/for singularity in \&self.singularities {/r patch_ping_ent.rs' src/core/engine/mod.rs

# Controller update
sed -i '/RightClick(f32, f32),/a\    SpawnRadarPing(f32, f32),' src/core/engine/controller.rs
cat << 'PING_CTRL' > patch_ping_ctrl.rs
                        EngineCommand::SpawnRadarPing(x, y) => {
                            worker_state.write().unwrap().spawn_radar_ping(x, y);
                        }
PING_CTRL
sed -i -e '/EngineCommand::RightClick/r patch_ping_ctrl.rs' src/core/engine/controller.rs

