use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{checkmark, table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Weapon {
    WoodenSword,
    BronzeSword,
    IronSword,
    SteelSword,
    MysticSword,
    WizardStaff,
}

impl Weapon {
    pub fn name(&self) -> &'static str {
        match self {
            Weapon::WoodenSword => "Wooden Sword",
            Weapon::BronzeSword => "Bronze Sword",
            Weapon::IronSword => "Iron Sword",
            Weapon::SteelSword => "Steel Sword",
            Weapon::MysticSword => "Mystic Sword",
            Weapon::WizardStaff => "Wizard Staff",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeaponData {
    pub owns: bool,
    pub equipped: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub flag: Weapon,
}

impl WeaponData {
    pub fn new(damage: usize, durability: usize, flag: Weapon) -> Self {
        Self {
            owns: false,
            equipped: false,
            damage,
            durability,
            default_durability: durability,
            flag,
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
        println!("Your {} broke!", self.flag.name());
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
            wooden_sword: WeaponData::new(10, 100, Weapon::WoodenSword),
            bronze_sword: WeaponData::new(20, 150, Weapon::BronzeSword),
            iron_sword: WeaponData::new(50, 200, Weapon::IronSword),
            steel_sword: WeaponData::new(200, 500, Weapon::SteelSword),
            mystic_sword: WeaponData::new(500, 1_000, Weapon::MysticSword),
            wizard_staff: WeaponData::new(1_000, 2_000, Weapon::WizardStaff),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(weapon: &WeaponData) -> String {
            format!(
                "{},{},{},{},{}",
                weapon.flag.name(),
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
