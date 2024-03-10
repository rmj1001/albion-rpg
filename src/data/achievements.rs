use serde::{Deserialize, Serialize};

use crate::{
    data::{player::Player, xp::XP},
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
        Self::default()
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

    /// Detects earned achievements, namely 1Mil gold and level 100
    pub fn check(player: &mut Player) {
        if player.bank.net_worth() >= 1_000_000 {
            player.achievements.earned_million_gold = true;
        }

        if XP::get_level(player.xp.total()) >= 100 {
            player.achievements.level_100_reached = true;
        }

        player.save();
    }
}
