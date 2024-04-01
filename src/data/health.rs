use serde::{Deserialize, Serialize};

use crate::prelude::{random_num, sleep, STANDARD_SLEEP};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub hp: usize,
    pub hunger: usize,
}

impl Default for Health {
    fn default() -> Self {
        Self { hp: 100, hunger: 0 }
    }
}

impl Health {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /**
    Only resets to default hp if less than 100 hp
    at the end of a battle so potions aren't wasted.
     */
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

        sleep(STANDARD_SLEEP);

        let new_health = random_num(1, 5);

        if new_health + self.hp > 100 {
            self.hp = 100;
            println!("Your health is fully restored!");
        } else {
            self.hp += new_health;
            println!("Your health has been restored {new_health} points.");
        }
    }
}
