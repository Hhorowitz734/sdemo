
mod subgroup;
mod interest_group;
mod society;
mod app;
mod issue;
mod ui;

use interest_group::InterestGroup;
use society::Society;
use issue::Issue;
use app::run_app;

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    /* SET UP INTEREST GROUPS*/
    let sports = InterestGroup::new(
        "Sports",
        0.8,
        vec!["Ping Pong", "Tennis", "Pickleball"]
    );

    let activism = InterestGroup::new(
        "Environmental Activism",
        0.4,
        vec!["Environmentalist", "Pro Development"]
    );

    /* SET UP ISSUES */
    let pollution = Issue::new(
        "Pollution",
        vec!["Environmental Activism".into()],
        HashMap::from([
            ("Environmentalist".to_string(), -0.1),
            ("Pro Development".to_string(), -0.03)
        ]),
    ); 
    
    /* SET UP SOCIETY */
    let society = Society::new(
        vec![sports, activism],
        vec![pollution]
    );

    run_app(&society)?;

    Ok(())
}

