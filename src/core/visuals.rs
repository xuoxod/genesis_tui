#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Dot,
    Diamond,
    Star,
    Triangle,
    Hexagon,
    Square,
    Cross,
    Clover,
    Spade,
    Heart,
    Vortex,
    Snowflake,
    Sparkle,
    Sun,
    Moon,
    Meteor,
    Target,
    Infinity,
}

impl Shape {
    pub fn as_str(&self) -> &'static str {
        match self {
            Shape::Dot => "●",
            Shape::Diamond => "♦",
            Shape::Star => "★",
            Shape::Triangle => "▲",
            Shape::Hexagon => "⬢",
            Shape::Square => "■",
            Shape::Cross => "✚",
            Shape::Clover => "♣",
            Shape::Spade => "♠",
            Shape::Heart => "♥",
            Shape::Vortex => "🌀",
            Shape::Snowflake => "❄",
            Shape::Sparkle => "✨",
            Shape::Sun => "☀",
            Shape::Moon => "☾",
            Shape::Meteor => "☄",
            Shape::Target => "◎",
            Shape::Infinity => "∞",
        }
    }
}
