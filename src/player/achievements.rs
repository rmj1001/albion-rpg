use serde::{Deserialize, Serialize};

use crate::utils::tui::{pretty_bool, table_from_csv};

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
    pub fn new() -> Self {
        Self {
            monsters_killed: 0,
            earned_million_gold: false,
            level_100_reached: false,
            stronghold_defeated: false,
            hacked_the_game: false,
        }
    }

    pub fn table(&self) {
        table_from_csv(vec![
            "Achievement,Data".to_string(),
            format!("Monsters Killed,{}", self.monsters_killed),
            format!("Stronghold Defeated?,{}", pretty_bool(self.stronghold_defeated)),
            format!("Earned One Million Gold?,{}", pretty_bool(self.earned_million_gold)),
            format!("Passed level 100?,{}", pretty_bool(self.level_100_reached)),
            format!("Hacked the Game?,{}", pretty_bool(self.hacked_the_game)),
        ])
    }
}
