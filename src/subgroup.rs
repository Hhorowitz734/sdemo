// src/subgroup.rs

#[derive(Debug)]
pub struct Subgroup {
    pub name: String,
    pub ratio: f64, // Proportion of subgroup within parent group
    pub mood: f64
}

impl Subgroup {
    pub fn new(name: impl Into<String>,
        ratio: f64,
        mood: f64) -> Self {
        Subgroup {
            name: name.into(),
            ratio,
            mood
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn apply_effect(&mut self, effect: f32) {
        self.mood += effect as f64;
    }
}
