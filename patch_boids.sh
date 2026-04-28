#!/bin/bash

# We want to insert Boids gathering before the mutable iteration in tick_internal
awk '
/for entity in &mut self\.entities \{/ {
    print "        let mut flock_data: Vec<(crate::core::grid::Position, crate::core::grid::Velocity)> = Vec::with_capacity(self.entities.len());"
    print "        for e_immut in &self.entities {"
    print "            flock_data.push((e_immut.position().clone(), e_immut.velocity().clone()));"
    print "        }"
    print "        for (i, entity) in self.entities.iter_mut().enumerate() {"
    next
}
/let pull = gravitational_pull/ {
    print "                let pull = crate::utils::physics::gravitational_pull(&singularity.position, &pos, singularity.mass, 2.0);"
    next
}
/vel.x \*= 0.98;/ {
    print "            // Boids logic"
    print "            let mut neighbors = Vec::new();"
    print "            for (j, (n_pos, n_vel)) in flock_data.iter().enumerate() {"
    print "                if i != j {"
    print "                    let dist = ((pos.x - n_pos.x).powi(2) + (pos.y - n_pos.y).powi(2)).sqrt();"
    print "                    if dist < 10.0 { // view radius"
    print "                        neighbors.push((n_pos.clone(), n_vel.clone()));"
    print "                    }"
    print "                }"
    print "            }"
    print "            let coh = crate::utils::physics::boids_cohesion(&pos, &neighbors);"
    print "            let ali = crate::utils::physics::boids_alignment(&vel, &neighbors);"
    print "            let sep = crate::utils::physics::boids_separation(&pos, &neighbors, 4.0);"
    print "            "
    print "            let boid_weight = 0.1;"
    print "            vel.x += (coh.x + ali.x + sep.x * 1.5) * boid_weight;"
    print "            vel.y += (coh.y + ali.y + sep.y * 1.5) * boid_weight;"
    print ""
    print $0
    next
}
{print}
' src/core/engine/mod.rs > temp.rs
mv temp.rs src/core/engine/mod.rs
cargo build
