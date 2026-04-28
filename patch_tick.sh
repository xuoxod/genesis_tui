cat << 'PATCH' > replace_tick.rs
    fn tick_internal(&mut self) {
        self.save_snapshot();
        self.tick_count += 1;
        let bounds_x = self.grid.width() as f32;
        let bounds_y = self.grid.height() as f32;
        let t_count = self.tick_count;
        let right_on = self.fence.is_active(FenceSide::Right);
        let left_on = self.fence.is_active(FenceSide::Left);
        let top_on = self.fence.is_active(FenceSide::Top);
        let bot_on = self.fence.is_active(FenceSide::Bottom);

        for entity in &mut self.entities {
            let pos = entity.position().clone();
            let mut vel = entity.velocity().clone();
            let mut zapped = false;

            if pos.x <= 0.0 {
                vel.x = vel.x.abs(); 
                if left_on { zapped = true; }
            } else if pos.x >= bounds_x {
                vel.x = -vel.x.abs();
                if right_on { zapped = true; }
            }

            if pos.y <= 0.0 {
                vel.y = vel.y.abs();
                if top_on { zapped = true; }
            } else if pos.y >= bounds_y {
                vel.y = -vel.y.abs();
                if bot_on { zapped = true; }
            }
            entity.set_velocity(vel);
            
            if zapped {
                entity.electrify(t_count as usize);
            }

            let constrain_pos = Position::new(
                (pos.x + vel.x).clamp(0.0, bounds_x),
                (pos.y + vel.y).clamp(0.0, bounds_y)
            );
            entity.set_position(constrain_pos);
        }
    }
PATCH
sed -i -e '/fn tick_internal(&mut self) {/,/        }/!b' -e '/fn tick_internal(&mut self) {/!d' -e '/fn tick_internal(&mut self) {/r replace_tick.rs' -e 'd' src/core/engine/mod.rs
