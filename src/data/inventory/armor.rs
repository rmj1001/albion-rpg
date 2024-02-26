use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{checkmark, table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Armor {
    Leather,
    Bronze,
    Iron,
    Steel,
    Dragonhide,
    Mystic,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArmorData {
    pub name: String,
    pub owns: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub equipped: bool,
}

impl ArmorData {
    pub fn new(name: &str, defense: usize, durability: usize) -> Self {
        Self {
            name: format!("{} Armor", name),
            owns: false,
            equipped: false,
            defense,
            durability,
            default_durability: durability,
        }
    }

    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);

        if self.durability < random_damage {
            self.break_armor();
        } else {
            self.durability -= random_damage;
        }
    }

    pub fn break_armor(&mut self) {
        println!("Your {} broke!", self.name);
        self.owns = false;
        self.durability = self.default_durability;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArmorInventory {
    pub leather: ArmorData,
    pub bronze: ArmorData,
    pub iron: ArmorData,
    pub steel: ArmorData,
    pub dragonhide: ArmorData,
    pub mystic: ArmorData,
}

impl ArmorInventory {
    pub fn new() -> ArmorInventory {
        ArmorInventory {
            leather: ArmorData::new("Leather", 10, 100),
            bronze: ArmorData::new("Bronze", 30, 200),
            iron: ArmorData::new("Iron", 50, 300),
            steel: ArmorData::new("Steel", 100, 500),
            dragonhide: ArmorData::new("Dragonhide", 200, 500),
            mystic: ArmorData::new("Mystic", 1_000, 10_000),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(armor: &ArmorData) -> String {
            format!(
                "{},{},{},{},{}",
                armor.name,
                checkmark(armor.owns),
                checkmark(armor.equipped),
                armor.defense,
                armor.durability,
            )
        }

        table_from_csv(vec![
            "Armor,Owned,Equipped,Defense,Durability".to_string(),
            entry(&self.leather),
            entry(&self.bronze),
            entry(&self.iron),
            entry(&self.steel),
            entry(&self.dragonhide),
            entry(&self.mystic),
        ])
    }

    pub fn get(&mut self, item_flag: &Armor) -> &mut ArmorData {
        match item_flag {
            Armor::Bronze => &mut self.bronze,
            Armor::Dragonhide => &mut self.dragonhide,
            Armor::Iron => &mut self.iron,
            Armor::Leather => &mut self.leather,
            Armor::Mystic => &mut self.mystic,
            Armor::Steel => &mut self.steel,
        }
    }

    /// For use in developer mode only
    pub fn toggle_own(&mut self, item_flag: &Armor) {
        let item = self.get(item_flag);
        item.owns = !item.owns;
    }
}
