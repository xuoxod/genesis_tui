            for ping in &self.radar_pings {
                let radius = ping.current_radius(t_count as usize);
                if ring_intersection(&pos, &ping.position, radius, 2.0) { // 2.0 thickness tolerance
                    entity.ping(t_count as usize);
                }
            }
