use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{checkmark, table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub owns: bool,
    pub equipped: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Weapon {
    pub fn new(name: &str, damage: usize, durability: usize) -> Self {
        Self {
            name: name.to_string(),
            owns: false,
            equipped: false,
            damage,
            durability,
            default_durability: durability,
        }
    }

    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);

        if self.durability < random_damage {
            self.break_weapon();
        } else {
            self.durability -= random_damage;
        }
    }

    pub fn break_weapon(&mut self) {
        println!("Your {} broke!", self.name);
        self.owns = false;
        self.durability = self.default_durability;
    }
}

pub enum WeaponItemFlag {
    WoodenSword,
    BronzeSword,
    IronSword,
    SteelSword,
    MysticSword,
    WizardStaff,
    InvalidItem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeaponsInventory {
    pub wooden_sword: Weapon,
    pub bronze_sword: Weapon,
    pub iron_sword: Weapon,
    pub steel_sword: Weapon,
    pub mystic_sword: Weapon,
    pub wizard_staff: Weapon,
}

impl WeaponsInventory {
    pub fn new() -> WeaponsInventory {
        WeaponsInventory {
            wooden_sword: Weapon::new("Wooden Sword", 10, 100),
            bronze_sword: Weapon::new("Bronze Sword", 20, 150),
            iron_sword: Weapon::new("Iron Sword", 50, 200),
            steel_sword: Weapon::new("Steel Rapier", 200, 500),
            mystic_sword: Weapon::new("Mystic Sword", 500, 1_000),
            wizard_staff: Weapon::new("Wizard Staff", 1_000, 2_000),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(weapon: &Weapon) -> String {
            format!(
                "{},{},{},{},{}",
                weapon.name,
                checkmark(weapon.owns),
                checkmark(weapon.equipped),
                weapon.damage,
                weapon.durability,
            )
        }
        table_from_csv(vec![
            "Weapon,Owned,Equipped,Damage,Durability,Buy Price,Sale Price".to_string(),
            entry(&self.wooden_sword),
            entry(&self.bronze_sword),
            entry(&self.iron_sword),
            entry(&self.steel_sword),
            entry(&self.mystic_sword),
            entry(&self.wizard_staff),
        ])
    }

    pub fn retrieve_item(&mut self, item_flag: WeaponItemFlag) -> Option<&mut Weapon> {
        match item_flag {
            WeaponItemFlag::BronzeSword => Some(&mut self.bronze_sword),
            WeaponItemFlag::IronSword => Some(&mut self.iron_sword),
            WeaponItemFlag::MysticSword => Some(&mut self.mystic_sword),
            WeaponItemFlag::SteelSword => Some(&mut self.steel_sword),
            WeaponItemFlag::WizardStaff => Some(&mut self.wizard_staff),
            WeaponItemFlag::WoodenSword => Some(&mut self.wooden_sword),
            WeaponItemFlag::InvalidItem => None,
        }
    }

    /// For use in developer mode only
    pub fn toggle_own(&mut self, item_flag: WeaponItemFlag) {
        let item_result = self.retrieve_item(item_flag);

        if item_result.is_none() {
            return;
        }

        if let Some(item) = item_result {
            item.owns = !item.owns;
        }
    }
}
