use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{checkmark, table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Weapon {
    WoodenSword,
    BronzeSword,
    IronSword,
    SteelSword,
    MysticSword,
    WizardStaff,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeaponData {
    pub name: String,
    pub owns: bool,
    pub equipped: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl WeaponData {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeaponsInventory {
    pub wooden_sword: WeaponData,
    pub bronze_sword: WeaponData,
    pub iron_sword: WeaponData,
    pub steel_sword: WeaponData,
    pub mystic_sword: WeaponData,
    pub wizard_staff: WeaponData,
}

impl WeaponsInventory {
    pub fn new() -> WeaponsInventory {
        WeaponsInventory {
            wooden_sword: WeaponData::new("Wooden Sword", 10, 100),
            bronze_sword: WeaponData::new("Bronze Sword", 20, 150),
            iron_sword: WeaponData::new("Iron Sword", 50, 200),
            steel_sword: WeaponData::new("Steel Rapier", 200, 500),
            mystic_sword: WeaponData::new("Mystic Sword", 500, 1_000),
            wizard_staff: WeaponData::new("Wizard Staff", 1_000, 2_000),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(weapon: &WeaponData) -> String {
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
            "Weapon,Owned,Equipped,Damage,Durability".to_string(),
            entry(&self.wooden_sword),
            entry(&self.bronze_sword),
            entry(&self.iron_sword),
            entry(&self.steel_sword),
            entry(&self.mystic_sword),
            entry(&self.wizard_staff),
        ])
    }

    pub fn get(&mut self, item_flag: &Weapon) -> &mut WeaponData {
        match item_flag {
            Weapon::BronzeSword => &mut self.bronze_sword,
            Weapon::IronSword => &mut self.iron_sword,
            Weapon::MysticSword => &mut self.mystic_sword,
            Weapon::SteelSword => &mut self.steel_sword,
            Weapon::WizardStaff => &mut self.wizard_staff,
            Weapon::WoodenSword => &mut self.wooden_sword,
        }
    }

    /// For use in developer mode only
    pub fn toggle_own(&mut self, item_flag: &Weapon) {
        let item = self.get(item_flag);

        item.owns = !item.owns;
    }
}
