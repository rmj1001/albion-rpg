use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::lib::tui::print_table;

#[derive(Serialize, Deserialize, Debug)]
pub struct Weapon {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Weapon {
    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability == 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeaponsInventory {
    pub wooden_sword: Weapon,
    pub bronze_sword: Weapon,
    pub iron_sword: Weapon,
    pub steel_sword: Weapon,
    pub mystic_sword: Weapon,
    pub wizard_staff: Weapon,
}

impl WeaponsInventory {
    pub fn print_table(&self) {
        print_table(vec![
            "Weapon,Purchased,Buy Price,Sale Price".to_string(),
            format!(
                "{},{},{},{}",
                self.wooden_sword.name,
                self.wooden_sword.owns,
                self.wooden_sword.price,
                self.wooden_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.bronze_sword.name,
                self.bronze_sword.owns,
                self.bronze_sword.price,
                self.bronze_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.iron_sword.name,
                self.iron_sword.owns,
                self.iron_sword.price,
                self.iron_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.steel_sword.name,
                self.steel_sword.owns,
                self.steel_sword.price,
                self.steel_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.mystic_sword.name,
                self.mystic_sword.owns,
                self.mystic_sword.price,
                self.mystic_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.wizard_staff.name,
                self.wizard_staff.owns,
                self.wizard_staff.price,
                self.wizard_staff.price / 2
            ),
        ])
    }
}
