// src/interest_group.rs

use rand::Rng;
use crate::subgroup::Subgroup;
use crate::issue::Issue;

use std::collections::HashSet;

#[derive(Debug)]
pub struct InterestGroup {
    pub name: String,
    pub population_ratio: f64, // Share of total population
    // Note that of course, population_ratio can overlap
    pub subgroups: Vec<Subgroup>,
    pub applicable_issues: HashSet<String> 
}


impl InterestGroup {
    pub fn new(
        name: impl Into<String>,
        population_ratio: f64,
        subgroup_names: Vec<impl Into<String>>,
    ) -> Self {
        let mut rng = rand::thread_rng();

        // Step 1 -> Random weights
        let raw_weights: Vec<f64> = (0..subgroup_names.len())
            .map(|_| rng.gen_range(0.1..1.0))
            .collect();

        let total: f64 = raw_weights.iter().sum();

        // Step 2 -> Normalize, create subgroups
        let subgroups = subgroup_names
            .into_iter()
            .zip(raw_weights)
            .map(|(name, weight)| {
                let ratio = weight / total;
                let mood = rng.gen_range(-1.0..=1.0); 
                Subgroup::new(name, ratio, mood)
            })
            .collect();

        InterestGroup {
            name: name.into(), 
            population_ratio,
            subgroups,
            applicable_issues: HashSet::new() 
        }
    }

    // Computes the weighted mood of this group
    pub fn weighted_mood(&self) -> f64 {
        self.subgroups
            .iter()
            .map(|subgroup| subgroup.ratio * subgroup.mood)
            .sum()
    }

    // Computes group's total contribution to society mood
    pub fn population_weighted_mood(&self) -> f64 {
        self.population_ratio * self.weighted_mood()
    }

    // Get subgroup by name if it exists
    pub fn get_subgroup(&self, name: &str) -> Option<&Subgroup> {
        self.subgroups.iter().find(|subgroup| subgroup.name == name)
    }

    // Calculate the effective population share of a specific subgroup
    // (population_ratio * subgroup.ratio)
    pub fn subgroup_population_share(&self, name: &str) -> Option<f64> {
        self.get_subgroup(name)
            .map(|sg| self.population_ratio * sg.ratio)
    }

    // Normalize subgroup ratios (if you want to manually fix them later)
    pub fn normalize_subgroups(&mut self) {
        let total: f64 = self.subgroups.iter().map(|sg| sg.ratio).sum();
        if total > 0.0 {
            for sg in &mut self.subgroups {
                sg.ratio /= total;
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn apply_issue(&mut self, issue: &Issue) {
        
        self.applicable_issues.insert(issue.get_name().to_string());

        for subgroup in &mut self.subgroups {
            let subgroup_name = subgroup.get_name();
            if let Some(effect) = issue.get_subgroup_effects(subgroup_name) {
                subgroup.apply_effect(effect);
            }
        }
    }
}

