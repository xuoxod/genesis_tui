        // High priority ping flash
        let ping_elapsed = current_tick.saturating_sub(self.pinged_start_tick);
        if self.pinged_start_tick > 0 && ping_elapsed <= 2 {
            return Some(("X", Vec3::new(255.0, 255.0, 255.0)));
        }

