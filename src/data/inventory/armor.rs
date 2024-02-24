use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::economy::armor::ArmorFlag;
use crate::utils::tui::{checkmark, table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub owns: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub equipped: bool,
}

impl Armor {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArmorInventory {
    pub leather: Armor,
    pub bronze: Armor,
    pub iron: Armor,
    pub steel: Armor,
    pub dragonhide: Armor,
    pub mystic: Armor,
}

impl ArmorInventory {
    pub fn new() -> ArmorInventory {
        ArmorInventory {
            leather: Armor::new("Leather", 10, 100),
            bronze: Armor::new("Bronze", 30, 200),
            iron: Armor::new("Iron", 50, 300),
            steel: Armor::new("Steel", 100, 500),
            dragonhide: Armor::new("Dragonhide", 200, 500),
            mystic: Armor::new("Mystic", 1_000, 10_000),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(armor: &Armor) -> String {
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

    pub fn retrieve_item(&mut self, item_flag: ArmorFlag) -> Option<&mut Armor> {
        match item_flag {
            ArmorFlag::Bronze => Some(&mut self.bronze),
            ArmorFlag::DragonHide => Some(&mut self.dragonhide),
            ArmorFlag::Iron => Some(&mut self.iron),
            ArmorFlag::Leather => Some(&mut self.leather),
            ArmorFlag::Mystic => Some(&mut self.mystic),
            ArmorFlag::Steel => Some(&mut self.steel),
            ArmorFlag::InvalidItem => None,
        }
    }

    /// For use in developer mode only
    pub fn toggle_own(&mut self, item_flag: ArmorFlag) {
        let item = self.retrieve_item(item_flag);

        if item.is_none() {
            return;
        }

        if let Some(item) = item {
            item.owns = !item.owns;
        }
    }
}
