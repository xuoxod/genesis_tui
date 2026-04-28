import sys
content = open('src/core/engine/mod.rs').read()
a = """    pub fn reset_entity_visuals(&mut self) {
        let colors = [
            crate::constants::colors::WHITE,
            crate::constants::colors::RED,
            crate::constants::colors::GREEN,
            crate::constants::colors::BLUE,
            crate::constants::colors::CYAN,
            crate::constants::colors::MAGENTA,
            crate::constants::colors::YELLOW,
            crate::constants::colors::ORANGE,
            crate::constants::colors::PURPLE,
            crate::constants::colors::TEAL,
            crate::constants::colors::NAVY,
            crate::constants::colors::LIME,
            crate::constants::colors::PINK,
            crate::constants::colors::GOLD,
        ];
        for ent in self.entities.iter_mut() {
            ent.reset_visuals();
        }
    }"""
b = """    pub fn reset_entity_visuals(&mut self) {
        for ent in self.entities.iter_mut() {
            ent.reset_visuals();
        }
    }"""

content = content.replace(a, b)
open('src/core/engine/mod.rs', 'w').write(content)
