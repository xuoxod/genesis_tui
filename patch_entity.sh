#!/bin/bash
awk '
/use glam::Vec3;/ {
    print $0
    print "use crate::core::visuals::Shape;"
    next
}
/    is_interacting: bool,/ {
    print "    // Generic Decoupled Visuals"
    print "    shape: Shape,"
    print "    base_color: Vec3,"
    print "    custom_gradient: Option<Vec<Vec3>>,"
    print ""
    print $0
    next
}
/            genome,/ {
    print $0
    print "            shape: Shape::Dot,"
    print "            base_color: colors::WHITE,"
    print "            custom_gradient: None,"
    next
}
/pub fn genome\(&self\) -> &Genome/ {
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
    print "        self.base_color = colors::WHITE;"
    print "        self.custom_gradient = None;"
    print "    }"
    next
}
{print}
' src/core/entity/mod.rs > temp.rs
mv temp.rs src/core/entity/mod.rs
echo "done patching entity"
