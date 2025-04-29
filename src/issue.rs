// src/issue.rs

use std::collections::HashMap;

#[derive(Debug)]
pub struct Issue {
    name: String,
    effect_recipients: Vec<String>,
    effects_map: HashMap<String, f32> 
}

impl Issue {
   
    // Basic Constructor
    pub fn new(
        name: impl Into<String>,
        effect_recipients: Vec<String>,
        effects_map: HashMap<String, f32>
    ) -> Self {
        Issue {
            name: name.into(),
            effect_recipients,
            effects_map
        }
    }

    pub fn get_effect_recipients(&self) -> &Vec<String> {
        &self.effect_recipients
    }

    pub fn get_subgroup_effects(&self, subgroup:&str) -> Option<f32> {
        self.effects_map.get(subgroup).copied()
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
