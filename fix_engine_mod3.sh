sed -i '/pub fn toggle_pause(&mut self)/i \    pub fn tick_count(&self) -> u64 {\n        self.tick_count\n    }\n' src/core/engine/mod.rs
