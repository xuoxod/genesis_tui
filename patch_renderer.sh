cat << 'PATCH' > replace_renderer.rs
        // Construct a professional enterprise-looking status bar
        let play_state = if e.is_paused() { "⏸ PAUSED " } else { "▶ RUNNING" };
        let play_color = if e.is_paused() { Color::Yellow } else { Color::LightGreen };

        let mut footer_content = vec![
            Span::styled(play_state, Style::default().fg(play_color).add_modifier(Modifier::BOLD)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("TICKS: {:0>6}", e.tick_count()), Style::default().fg(Color::Cyan)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("ENTITIES: {:0>3}", e.entities().len()), Style::default().fg(Color::Magenta)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("SPEED: {}ms/T ", tick_rate), Style::default().fg(Color::White)),
            Span::styled(" || ", Style::default().fg(Color::DarkGray)),
        ];
        
        // Add fence indicators
        let f = e.fence();
        let fs = crate::utils::fence::FenceSide::Top;
        footer_content.push(Span::styled("FENCE [1-4, 5=All]: ", Style::default().fg(Color::Blue)));
        let t_color = if f.is_active(crate::utils::fence::FenceSide::Top) { Color::Cyan } else { Color::DarkGray };
        let b_color = if f.is_active(crate::utils::fence::FenceSide::Bottom) { Color::Cyan } else { Color::DarkGray };
        let l_color = if f.is_active(crate::utils::fence::FenceSide::Left) { Color::Cyan } else { Color::DarkGray };
        let r_color = if f.is_active(crate::utils::fence::FenceSide::Right) { Color::Cyan } else { Color::DarkGray };
        footer_content.push(Span::styled("T ", Style::default().fg(t_color)));
        footer_content.push(Span::styled("B ", Style::default().fg(b_color)));
        footer_content.push(Span::styled("L ", Style::default().fg(l_color)));
        footer_content.push(Span::styled("R ", Style::default().fg(r_color)));
        
        footer_content.push(Span::styled("|| ", Style::default().fg(Color::DarkGray)));
        footer_content.push(Span::styled("[Q]uit ", Style::default().fg(Color::LightRed)));
        footer_content.push(Span::styled("[SPC]Pause ", Style::default().fg(Color::White)));
        footer_content.push(Span::styled("[R]eset ", Style::default().fg(Color::White)));
        footer_content.push(Span::styled("[←/→]Scrub ", Style::default().fg(Color::White)));
        footer_content.push(Span::styled("[↑/↓]Speed", Style::default().fg(Color::White)));
PATCH
sed -i -e '/let play_state =/,/Span::styled("\[↑\/↓\]Speed", Style::default().fg(Color::White)),/c\' -e '' src/ui/renderer.rs
sed -i '/\];/r replace_renderer.rs' src/ui/renderer.rs
