#!/bin/bash
awk '
/for ent in e\.entities\(\) \{/ {
    print $0
    print "                    // Trail rendering (Phosphorescent Trails)"
    print "                    let trail_len = ent.trail().len();"
    print "                    for (i, t_pos) in ent.trail().iter().enumerate() {"
    print "                        let intensity = (i as f32 / trail_len as f32).max(0.1);"
    print "                        let color = ratatui::style::Color::Rgb("
    print "                            (0.0 * intensity) as u8,"
    print "                            (255.0 * intensity) as u8,"
    print "                            (200.0 * intensity) as u8"
    print "                        );"
    print "                        ctx.print(t_pos.x as f64, t_pos.y as f64, ratatui::text::Span::styled(\".\", ratatui::style::Style::default().fg(color)));"
    print "                    }"
    next
}
{print}
' src/ui/renderer.rs > temp.rs
mv temp.rs src/ui/renderer.rs
cargo build
