cat << 'CODE' > src/ui/renderer.rs
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

        let c = Canvas::default().block(Block::default().borders(Borders::ALL).title(" 🧬 Genesis Engine • TUI Interactive Console "))
            .x_bounds([0.0, e.grid().width() as f64])
            .y_bounds([0.0, e.grid().height() as f64])
            .paint(|ctx| {
                for ent in e.entities() {
                    // Use a more organic or dynamic character
                    ctx.print(ent.position().x as f64, ent.position().y as f64, "●");
                }
            });
            
        f.render_widget(c, chunks[0]);

        // Construct a professional enterprise-looking status bar
        let play_state = if e.is_paused() { "⏸ PAUSED " } else { "▶ RUNNING" };
        let play_color = if e.is_paused() { Color::Yellow } else { Color::LightGreen };

        let footer_content = vec![
            Span::styled(play_state, Style::default().fg(play_color).add_modifier(Modifier::BOLD)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("TICKS: {:0>6}", e.tick_count()), Style::default().fg(Color::Cyan)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("ENTITIES: {:0>3}", e.entities().len()), Style::default().fg(Color::Magenta)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("SPEED: {}ms/T ", tick_rate), Style::default().fg(Color::White)),
            Span::styled(" || ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Q]uit ", Style::default().fg(Color::LightRed)),
            Span::styled("[SPC]Pause ", Style::default().fg(Color::White)),
            Span::styled("[R]eset ", Style::default().fg(Color::White)),
            Span::styled("[←/→]Scrub ", Style::default().fg(Color::White)),
            Span::styled("[↑/↓]Speed", Style::default().fg(Color::White)),
        ];

        let footer = Paragraph::new(Line::from(footer_content))
            .style(Style::default().bg(Color::Black));
            
        f.render_widget(footer, chunks[1]);
    }
}
CODE
