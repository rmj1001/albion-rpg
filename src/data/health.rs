use serde::{Deserialize, Serialize};

use crate::utils::{math::random_num, tui::sleep};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Health {
    pub hp: usize,
    pub hunger: usize,
}

impl Health {
    pub fn new() -> Self {
        Self {
            hp: 100,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Only resets to default hp if less than 100 hp at the end of a battle
    /// so potions aren't wasted.
    pub fn restore(&mut self) {
        if self.hp < 100 {
            self.hp = 100;
        }

        if self.hunger > 0 {
            self.hunger = 0;
        }
    }

    pub fn heal(&mut self) {
        if !self.hunger == 0 || self.hp >= 100 {
            return;
        }

        println!("Healing...");

        sleep(1);

        let new_health = random_num(1, 5);

        if new_health + self.hp > 100 {
            self.hp = 100;
            println!("Your health is fully restored!");
        } else {
            self.hp += new_health;
            println!("Your health has been restored {} points.", new_health);
        }
    }
}
