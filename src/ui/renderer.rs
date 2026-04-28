use crate::core::engine::Engine;
use crate::utils::fence::FenceSide;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line as TextLine, Span},
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, BorderType, Borders, Paragraph,
    },
    Frame,
};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, f: &mut Frame, e: &Engine, tick_rate: u64) {
        let size = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Min(0),    // Main canvas
                Constraint::Length(1), // Separator / Footer
            ])
            .split(size);

        let fen = e.fence();

        let mut active_borders = Borders::NONE;
        if fen.is_active(FenceSide::Top) {
            active_borders |= Borders::TOP;
        }
        if fen.is_active(FenceSide::Bottom) {
            active_borders |= Borders::BOTTOM;
        }
        if fen.is_active(FenceSide::Left) {
            active_borders |= Borders::LEFT;
        }
        if fen.is_active(FenceSide::Right) {
            active_borders |= Borders::RIGHT;
        }

        let any_active = active_borders != Borders::NONE;

        let tick = e.tick_count() as usize;
        let mut border_color = Color::DarkGray;
        let mut border_type = BorderType::Plain;
        let mut title = " AetherFlux NexusPipe • TUI Interactive Console ";

        if any_active {
            border_type = BorderType::Double;

            // "Nerdy" strobe effect based on the Eulerian tick
            let cycle = tick % 10;
            border_color = match cycle {
                0..=2 => Color::Magenta,
                3..=5 => Color::Cyan,
                6..=7 => Color::White,
                _ => Color::LightBlue,
            };
            title = " ⚡ [HIGH VOLTAGE] AetherFlux NexusPipe ⚡ ";
        }

        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(if any_active {
                BorderType::Double
            } else {
                BorderType::Plain
            })
            .border_style(Style::default().fg(if any_active {
                Color::DarkGray
            } else {
                Color::DarkGray
            }))
            .title(Span::styled(
                title,
                Style::default().fg(if any_active {
                    border_color
                } else {
                    Color::White
                }),
            ));

        let c = Canvas::default()
            .block(main_block)
            .x_bounds([0.0, e.grid().width() as f64])
            .y_bounds([0.0, e.grid().height() as f64])
            .paint(|ctx| {
                let current_tick = e.tick_count() as usize;

                let f_obj = e.fence();
                let gw = e.grid().width() as f64;
                let gh = e.grid().height() as f64;

                // Draw actual electric fence boundary lines directly on the physics plane
                let cycle = current_tick % 10;
                let hot_color = match cycle {
                    0..=3 => Color::Magenta,
                    4..=6 => Color::Cyan,
                    _ => Color::White,
                };

                if f_obj.is_active(FenceSide::Bottom) {
                    ctx.draw(&CanvasLine {
                        x1: 0.0,
                        y1: 0.0,
                        x2: gw,
                        y2: 0.0,
                        color: hot_color,
                    });
                    ctx.draw(&CanvasLine {
                        x1: 0.0,
                        y1: 1.0,
                        x2: gw,
                        y2: 1.0,
                        color: Color::Yellow,
                    });
                }
                if f_obj.is_active(FenceSide::Top) {
                    ctx.draw(&CanvasLine {
                        x1: 0.0,
                        y1: gh,
                        x2: gw,
                        y2: gh,
                        color: hot_color,
                    });
                    ctx.draw(&CanvasLine {
                        x1: 0.0,
                        y1: gh - 1.0,
                        x2: gw,
                        y2: gh - 1.0,
                        color: Color::Yellow,
                    });
                }
                if f_obj.is_active(FenceSide::Left) {
                    ctx.draw(&CanvasLine {
                        x1: 0.0,
                        y1: 0.0,
                        x2: 0.0,
                        y2: gh,
                        color: hot_color,
                    });
                    ctx.draw(&CanvasLine {
                        x1: 1.0,
                        y1: 0.0,
                        x2: 1.0,
                        y2: gh,
                        color: Color::Yellow,
                    });
                }
                if f_obj.is_active(FenceSide::Right) {
                    ctx.draw(&CanvasLine {
                        x1: gw,
                        y1: 0.0,
                        x2: gw,
                        y2: gh,
                        color: hot_color,
                    });
                    ctx.draw(&CanvasLine {
                        x1: gw - 1.0,
                        y1: 0.0,
                        x2: gw - 1.0,
                        y2: gh,
                        color: Color::Yellow,
                    });
                }

                for s in e.singularities() {
                    let s_cycle = current_tick % 5;
                    let mut s_char = "O";
                    if s_cycle == 0 {
                        s_char = "+";
                    } else if s_cycle == 1 {
                        s_char = "*";
                    } else if s_cycle == 2 {
                        s_char = "o";
                    }
                    ctx.print(
                        s.position.x as f64,
                        s.position.y as f64,
                        Span::styled(
                            s_char,
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::BOLD),
                        ),
                    );
                }

                for ping in e.radar_pings() {
                    let radius = ping.current_radius(current_tick) as f64;
                    if radius > 0.0 {
                        ctx.draw(&Circle {
                            x: ping.position.x as f64,
                            y: ping.position.y as f64,
                            radius,
                            color: Color::Green,
                        });
                    }
                }

                for ent in e.entities() {
                    // Trail rendering (Phosphorescent Trails)
                    let trail_len = ent.trail().len();
                    for (i, t_pos) in ent.trail().iter().enumerate() {
                        let intensity = (i as f32 / trail_len as f32).max(0.1);
                        let color = ratatui::style::Color::Rgb(
                            (0.0 * intensity) as u8,
                            (255.0 * intensity) as u8,
                            (200.0 * intensity) as u8,
                        );
                        ctx.print(
                            t_pos.x as f64,
                            t_pos.y as f64,
                            ratatui::text::Span::styled(
                                ".",
                                ratatui::style::Style::default().fg(color),
                            ),
                        );
                    }
                    let mut render_char = "●";
                    let mut render_color = Color::Reset;

                    if let Some((frame, color)) = ent.get_render_effect(current_tick) {
                        render_char = frame;
                        // Map the glam::Vec3 back to Ratatui layout colors
                        render_color = Color::Rgb(color.x as u8, color.y as u8, color.z as u8);
                    }

                    // We use ctx.print since Canvas expects points
                    ctx.print(
                        ent.position().x as f64,
                        ent.position().y as f64,
                        Span::styled(render_char, Style::default().fg(render_color)),
                    );
                }
            });

        f.render_widget(c, chunks[0]);

        // Construct a professional enterprise-looking status bar
        let play_state = if e.is_paused() {
            "⏸ PAUSED "
        } else {
            "▶ RUNNING"
        };
        let play_color = if e.is_paused() {
            Color::Yellow
        } else {
            Color::LightGreen
        };

        let mut footer_content = vec![
            Span::styled(
                play_state,
                Style::default().fg(play_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("TICKS: {:0>6}", e.tick_count()),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("ENTITIES: {:0>3}", e.entities().len()),
                Style::default().fg(Color::Magenta),
            ),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("SPEED: {}ms/T ", tick_rate),
                Style::default().fg(Color::White),
            ),
            Span::styled(" || ", Style::default().fg(Color::DarkGray)),
        ];

        // Add fence indicators
        footer_content.push(Span::styled(
            "FENCE [1-4, 5=All]: ",
            Style::default().fg(Color::Blue),
        ));
        let t_color = if fen.is_active(FenceSide::Top) {
            Color::LightCyan
        } else {
            Color::DarkGray
        };
        let b_color = if fen.is_active(FenceSide::Bottom) {
            Color::LightCyan
        } else {
            Color::DarkGray
        };
        let l_color = if fen.is_active(FenceSide::Left) {
            Color::LightCyan
        } else {
            Color::DarkGray
        };
        let r_color = if fen.is_active(FenceSide::Right) {
            Color::LightCyan
        } else {
            Color::DarkGray
        };
        footer_content.push(Span::styled(
            if fen.is_active(FenceSide::Top) {
                "[T]"
            } else {
                " T "
            },
            Style::default()
                .fg(t_color)
                .add_modifier(if fen.is_active(FenceSide::Top) {
                    Modifier::BOLD | Modifier::REVERSED
                } else {
                    Modifier::empty()
                }),
        ));
        footer_content.push(Span::styled(
            if fen.is_active(FenceSide::Bottom) {
                "[B]"
            } else {
                " B "
            },
            Style::default()
                .fg(b_color)
                .add_modifier(if fen.is_active(FenceSide::Bottom) {
                    Modifier::BOLD | Modifier::REVERSED
                } else {
                    Modifier::empty()
                }),
        ));
        footer_content.push(Span::styled(
            if fen.is_active(FenceSide::Left) {
                "[L]"
            } else {
                " L "
            },
            Style::default()
                .fg(l_color)
                .add_modifier(if fen.is_active(FenceSide::Left) {
                    Modifier::BOLD | Modifier::REVERSED
                } else {
                    Modifier::empty()
                }),
        ));
        footer_content.push(Span::styled(
            if fen.is_active(FenceSide::Right) {
                "[R]"
            } else {
                " R "
            },
            Style::default()
                .fg(r_color)
                .add_modifier(if fen.is_active(FenceSide::Right) {
                    Modifier::BOLD | Modifier::REVERSED
                } else {
                    Modifier::empty()
                }),
        ));

        footer_content.push(Span::styled(" || ", Style::default().fg(Color::DarkGray)));
        footer_content.push(Span::styled(
            "[Q]uit ",
            Style::default().fg(Color::LightRed),
        ));
        footer_content.push(Span::styled(
            "[SPC]Pause ",
            Style::default().fg(Color::White),
        ));
        footer_content.push(Span::styled("[R]eset ", Style::default().fg(Color::White)));
        footer_content.push(Span::styled(
            "[←/→]Scrub ",
            Style::default().fg(Color::White),
        ));
        footer_content.push(Span::styled(
            "[↑/↓]Speed",
            Style::default().fg(Color::White),
        ));

        let footer =
            Paragraph::new(TextLine::from(footer_content)).style(Style::default().bg(Color::Black));

        f.render_widget(footer, chunks[1]);
    }
}
