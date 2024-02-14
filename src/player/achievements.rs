use serde::{Deserialize, Serialize};

use crate::utils::tui::{pretty_bool, print_table};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Achievements {
    // Determined in combat
    pub monsters_killed: usize,
    pub stronghold_defeated: bool,

    // Determined when achievements viewed
    pub earned_million_gold: bool,
    pub level_100_reached: bool,
    pub hacked_the_game: bool,
}

impl Achievements {
    pub fn print_table(&self) {
        print_table(vec![
            "Achievement,Data".to_string(),
            format!("Monsters Killed,{}", self.monsters_killed),
            format!("Stronghold Defeated?,{}", pretty_bool(self.stronghold_defeated)),
            format!("Earned One Million Gold?,{}", pretty_bool(self.earned_million_gold)),
            format!("Passed level 100?,{}", pretty_bool(self.level_100_reached)),
            format!("Hacked the Game?,{}", pretty_bool(self.hacked_the_game)),
        ])
    }
}
