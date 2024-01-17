use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::lib::tui::{pretty_bool, print_table};

#[derive(Serialize, Deserialize, Debug)]
pub struct Armor {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub equipped: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Armor {
    pub fn decrease_durability(&mut self) {
        let random_damage = rand::thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability == 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmorInventory {
    pub leather: Armor,
    pub bronze: Armor,
    pub iron: Armor,
    pub steel: Armor,
    pub dragonhide: Armor,
    pub mystic: Armor,
}

impl ArmorInventory {
    pub fn print_table(&self) {
        print_table(vec![
            "Armor,Purchased,Equipped,Buy Price,Sale Price".to_string(),
            format!(
                "{},{},{},{},{}",
                self.leather.name,
                pretty_bool(self.leather.owns),
                pretty_bool(self.leather.equipped),
                self.leather.price,
                self.leather.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.bronze.name,
                pretty_bool(self.bronze.owns),
                pretty_bool(self.bronze.equipped),
                self.bronze.price,
                self.bronze.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.iron.name,
                pretty_bool(self.iron.owns),
                pretty_bool(self.iron.equipped),
                self.iron.price,
                self.iron.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.steel.name,
                pretty_bool(self.steel.owns),
                pretty_bool(self.steel.equipped),
                self.steel.price,
                self.steel.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.dragonhide.name,
                pretty_bool(self.dragonhide.owns),
                pretty_bool(self.dragonhide.equipped),
                self.dragonhide.price,
                self.dragonhide.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.mystic.name,
                pretty_bool(self.mystic.owns),
                pretty_bool(self.mystic.equipped),
                self.mystic.price,
                self.mystic.price / 2
            ),
        ])
    }
}
