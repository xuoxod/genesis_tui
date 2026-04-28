        self.radar_pings.retain(|p| p.is_active(t_count as usize));
