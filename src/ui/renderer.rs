use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
    Frame,
};
use crate::core::engine::Engine;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self { Self }

    pub fn draw(&self, f: &mut Frame, e: &Engine, tick_rate: u64) {
        let size = f.area();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Min(0),      // Main canvas
                Constraint::Length(1),   // Separator / Footer
            ])
            .split(size);

        let c = Canvas::default().block(Block::default().borders(Borders::ALL).title(" ⚡ AetherFlux NexusPipe • TUI Interactive Console "))
            .x_bounds([0.0, e.grid().width() as f64])
            .y_bounds([0.0, e.grid().height() as f64])
            .paint(|ctx| {
                let current_tick = e.tick_count() as usize;

                // Render electric fence boundaries
                // Note: The Canvas widget restricts us points/lines easily, so we just use the UI to print labels
                let f_obj = e.fence();

                for ent in e.entities() {
                    let mut render_char = "●";
                    let mut render_color = Color::Reset;

                    if let Some((frame, color)) = ent.get_render_effect(current_tick) {
                        render_char = frame;
                        // Map the glam::Vec3 back to Ratatui layout colors
                        render_color = Color::Rgb(color.x as u8, color.y as u8, color.z as u8);
                    }

                    // We use ctx.print since Canvas expects points
                    ctx.print(ent.position().x as f64, ent.position().y as f64, Span::styled(render_char, Style::default().fg(render_color)));
                }
            });
            
        f.render_widget(c, chunks[0]);

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
        let fen = e.fence();
        footer_content.push(Span::styled("FENCE [1-4, 5=All]: ", Style::default().fg(Color::Blue)));
        let t_color = if fen.is_active(crate::utils::fence::FenceSide::Top) { Color::Cyan } else { Color::DarkGray };
        let b_color = if fen.is_active(crate::utils::fence::FenceSide::Bottom) { Color::Cyan } else { Color::DarkGray };
        let l_color = if fen.is_active(crate::utils::fence::FenceSide::Left) { Color::Cyan } else { Color::DarkGray };
        let r_color = if fen.is_active(crate::utils::fence::FenceSide::Right) { Color::Cyan } else { Color::DarkGray };
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

        let footer = Paragraph::new(Line::from(footer_content))
            .style(Style::default().bg(Color::Black));
            
        f.render_widget(footer, chunks[1]);
    }
}
