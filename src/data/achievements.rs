use serde::{Deserialize, Serialize};

use crate::utils::tui::{checkmark, table_from_csv};

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

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        table_from_csv(vec![
            "Achievement,Data".to_string(),
            format!("Monsters Killed,{}", self.monsters_killed),
            format!("Stronghold Defeated?,{}", checkmark(self.stronghold_defeated)),
            format!("Earned One Million Gold?,{}", checkmark(self.earned_million_gold)),
            format!("Passed level 100?,{}", checkmark(self.level_100_reached)),
            format!("Hacked the Game?,{}", checkmark(self.hacked_the_game)),
        ])
    }
}
