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
