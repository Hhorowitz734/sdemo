// src/game.rs

use crate::society::Society;
use crate::issue::Issue;

pub struct Game {
    pub society: Society,
    pub turn: u32
}

impl Game {

    pub fn new(
        society: Society,
    ) -> Self {
        Game {
            society,
            turn: 0
        }
    }

    pub fn next_turn(&mut self) {
        
        // ==== APPLY ISSUES ====
        self.society.apply_issues();

        // ==== UPDATE TURN ====
        self.turn += 1;
    }
}
