use glam::Vec3;

// --------------------------------------------------------
// THE GENESIS EXTENDED ENTERPRISE PALETTE (Float RGB Vectors)
// --------------------------------------------------------

// Core Neutral Palette
pub const WHITE: Vec3 = Vec3::new(255.0, 255.0, 255.0);
pub const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const GRAY_DARK: Vec3 = Vec3::new(64.0, 64.0, 64.0);
pub const GRAY_LIGHT: Vec3 = Vec3::new(192.0, 192.0, 192.0);

// Primary Additive Colors
pub const RED: Vec3 = Vec3::new(255.0, 0.0, 0.0);
pub const GREEN: Vec3 = Vec3::new(0.0, 255.0, 0.0);
pub const BLUE: Vec3 = Vec3::new(0.0, 0.0, 255.0);

// Extended Vibrant Spectrum
pub const CYAN: Vec3 = Vec3::new(0.0, 255.0, 255.0);
pub const MAGENTA: Vec3 = Vec3::new(255.0, 0.0, 255.0);
pub const YELLOW: Vec3 = Vec3::new(255.0, 255.0, 0.0);
pub const ORANGE: Vec3 = Vec3::new(255.0, 165.0, 0.0);
pub const PURPLE: Vec3 = Vec3::new(128.0, 0.0, 128.0);
pub const TEAL: Vec3 = Vec3::new(0.0, 128.0, 128.0);
pub const NAVY: Vec3 = Vec3::new(0.0, 0.0, 128.0);
pub const LIME: Vec3 = Vec3::new(50.0, 205.0, 50.0);
pub const PINK: Vec3 = Vec3::new(255.0, 105.0, 180.0);
pub const GOLD: Vec3 = Vec3::new(255.0, 215.0, 0.0);
pub const INDIGO: Vec3 = Vec3::new(75.0, 0.0, 130.0);
pub const SLATE: Vec3 = Vec3::new(112.0, 128.0, 144.0);
pub const CRIMSON: Vec3 = Vec3::new(220.0, 20.0, 60.0);
pub const EMERALD: Vec3 = Vec3::new(80.0, 200.0, 120.0);

// --------------------------------------------------------
// SEMANTIC CONTEXT BINDINGS (SST)
// --------------------------------------------------------

pub const HIGHLIGHT_FOCUS: Vec3 = YELLOW;
pub const SELECTED_ENTITY: Vec3 = CYAN;
pub const MUTATION_ACTIVE: Vec3 = MAGENTA;
pub const DANGER_WARN: Vec3 = ORANGE;
pub const DEAD_DARK: Vec3 = SLATE;

// Animation Configs
pub const DEFAULT_LERP_SPEED: f32 = 0.05; // 5% progression per tick
pub const FAST_LERP_SPEED: f32 = 0.20; // 20% progression per tick
