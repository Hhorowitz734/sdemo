// src/society.rs

use crate::interest_group::InterestGroup;
use crate::issue::Issue;

#[derive(Debug)]
pub struct Society {
    pub interest_groups: Vec<InterestGroup>,
    pub issues: Vec<Issue>
}


impl Society {

    // Create a new society given a list of interest groups
    pub fn new(interest_groups: Vec<InterestGroup>,
        issues: Vec<Issue> ) -> Self {
        Society { interest_groups, issues }
    }

    // Compute overall mood of society
    pub fn overall_mood(&self) -> f64 {
        self.interest_groups
            .iter()
            .map(|group| group.population_weighted_mood())
            .sum()
    }

    // Print a quick summary
    pub fn print_summary(&self) {
            println!("Society Summary:");
            for group in &self.interest_groups {
                println!(
                    "- {} ({}% of population): weighted mood = {:.2}",
                    group.name,
                    group.population_ratio * 100.0,
                    group.weighted_mood()
                );
            }
            println!(
                "\nOverall Society Mood: {:.2}",
                self.overall_mood()
            );
        }

    pub fn apply_issues(&mut self) {
        
        for issue in &self.issues {

            // Find which groups it affects
            for recipient_name in issue.get_effect_recipients() {
                for group in &mut self.interest_groups {
                    if group.get_name() == recipient_name {
                        group.apply_issue(issue);
                    }
                }
            }
        }
    }

}
