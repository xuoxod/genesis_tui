import sys
content = open('src/core/engine/mod.rs').read()

import re

# replace shapes block
content = re.sub(
    r'let shapes = \[\s*.*?\s*\];',
    'let shapes = [\n        Shape::Dot, Shape::Diamond, Shape::Star, Shape::Triangle,\n        Shape::Hexagon, Shape::Square, Shape::Cross, Shape::Clover,\n        Shape::Spade, Shape::Heart, Shape::Vortex, Shape::Snowflake,\n        Shape::Sparkle, Shape::Sun, Shape::Moon, Shape::Meteor,\n        Shape::Target, Shape::Infinity\n    ];',
    content,
    flags=re.DOTALL
)

print(content[:500])
content = content.replace("crate::constants::colors::PALETTE", "colors")

content = content.replace("crate::utils::color_math::generate_gradient", "crate::utils::color::generate_gradient")

colors_array = """        let colors = [
            crate::constants::colors::WHITE, crate::constants::colors::RED,
            crate::constants::colors::GREEN, crate::constants::colors::BLUE,
            crate::constants::colors::CYAN, crate::constants::colors::MAGENTA,
            crate::constants::colors::YELLOW, crate::constants::colors::ORANGE,
            crate::constants::colors::PURPLE, crate::constants::colors::TEAL,
            crate::constants::colors::NAVY, crate::constants::colors::LIME,
            crate::constants::colors::PINK, crate::constants::colors::GOLD,
        ];
        for ent in self.entities.iter_mut() {"""

content = content.replace("for ent in self.entities.iter_mut() {", colors_array)
open('src/core/engine/mod.rs', 'w').write(content)
