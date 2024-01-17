use serde::{Deserialize, Serialize};

use crate::lib::tui::print_table;

#[derive(Serialize, Deserialize, Debug)]
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
        fn prettify<'a>(flag: bool) -> &'a str {
            match flag {
                true => "Yes",
                false => "No",
            }
        }

        print_table(vec![
            "Achievement,Data".to_string(),
            format!("Monsters Killed,{}", self.monsters_killed),
            format!(
                "Stronghold Defeated?,{}",
                prettify(self.stronghold_defeated)
            ),
            format!(
                "Earned One Million Gold?,{}",
                prettify(self.earned_million_gold)
            ),
            format!("Passed level 100?,{}", prettify(self.level_100_reached)),
            format!("Hacked the Game?,{}", prettify(self.hacked_the_game)),
        ])
    }
}
