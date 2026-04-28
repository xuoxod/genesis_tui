#!/bin/bash
awk '
/pub fn genome\(&self\) -> &Genome \{ &self.genome \}/ {
    print $0
    print "    pub fn shape(&self) -> Shape { self.shape }"
    print "    pub fn shape_char(&self) -> &'\''static str { self.shape.as_str() }"
    print "    pub fn base_color(&self) -> Vec3 { self.base_color }"
    print "    pub fn set_shape(&mut self, shape: Shape) { self.shape = shape; }"
    print "    pub fn set_base_color(&mut self, color: Vec3) { self.base_color = color; }"
    print "    pub fn apply_custom_gradient(&mut self, gradient: Vec<Vec3>) { self.custom_gradient = Some(gradient); }"
    print "    pub fn has_custom_gradient(&self) -> bool { self.custom_gradient.is_some() }"
    print "    pub fn custom_gradient(&self) -> Option<&Vec<Vec3>> { self.custom_gradient.as_ref() }"
    print "    pub fn reset_visuals(&mut self) {"
    print "        self.shape = Shape::Dot;"
    print "        self.base_color = crate::constants::colors::WHITE;"
    print "        self.custom_gradient = None;"
    print "    }"
    next
}
{print}
' src/core/entity/mod.rs > temp_entity.rs
mv temp_entity.rs src/core/entity/mod.rs

sed -i 's/Genome::new()/Genome::new_random(10)/g' tests/visuals_core.rs
sed -i 's/use genesis_tui::core::entity::{Entity, Shape};/use genesis_tui::core::entity::Entity;\nuse genesis_tui::core::visuals::Shape;/g' tests/visuals_core.rs
