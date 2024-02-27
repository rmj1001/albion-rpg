use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{checkmark, table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Armor {
    Leather,
    Bronze,
    Iron,
    Steel,
    Dragonhide,
    Mystic,
}

impl Armor {
    pub fn name(&self) -> &'static str {
        match self {
            Armor::Leather => "Leather Armor",
            Armor::Bronze => "Bronze Armor",
            Armor::Iron => "Iron Armor",
            Armor::Steel => "Steel Armor",
            Armor::Dragonhide => "Dragonhide Armor",
            Armor::Mystic => "Mystic Armor",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArmorData {
    pub owns: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub equipped: bool,
    pub flag: Armor,
}

impl ArmorData {
    pub fn new(defense: usize, durability: usize, flag: Armor) -> Self {
        Self {
            owns: false,
            equipped: false,
            defense,
            durability,
            default_durability: durability,
            flag,
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
        println!("Your {} broke!", self.flag.name());
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
            leather: ArmorData::new(10, 100, Armor::Leather),
            bronze: ArmorData::new(30, 200, Armor::Bronze),
            iron: ArmorData::new(50, 300, Armor::Iron),
            steel: ArmorData::new(100, 500, Armor::Steel),
            dragonhide: ArmorData::new(200, 500, Armor::Dragonhide),
            mystic: ArmorData::new(1_000, 10_000, Armor::Mystic),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(armor: &ArmorData) -> String {
            format!(
                "{},{},{},{},{}",
                armor.flag.name(),
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
