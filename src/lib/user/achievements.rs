use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Achievements {
    // Determined in combat
    pub monsters_killed: u32,
    pub stronghold_defeated: bool,

    // Determined when achievements viewed
    pub earned_million_gold: bool,
    pub level_100_reached: bool,
    pub hacked_the_game: bool,
}
