use crate::utils::tui::{checkmark, csv_table};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

use crate::data::player::Player;
use crate::prelude::{select, InventoryError};
use std::result::Result;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Weapon {
    Wooden,
    Bronze,
    Iron,
    Steel,
    Mystic,
    WizardStaff,
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::WizardStaff => "Wizard Staff".to_string(),
            sword => format!("{:?} Sword", sword),
        };

        write!(f, "{string}")
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

impl Default for WeaponData {
    fn default() -> Self {
        Self {
            owns: false,
            equipped: false,
            damage: 5,
            durability: 50,
            default_durability: 50,
            flag: Weapon::Wooden,
        }
    }
}

impl Display for WeaponData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.flag,
            checkmark(self.owns),
            checkmark(self.equipped),
            self.damage,
            self.durability,
        )
    }
}

impl WeaponData {
    pub fn new(damage: usize, durability: usize, flag: Weapon) -> Self {
        Self {
            damage,
            durability,
            flag,
            ..Default::default()
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
        println!("Your {} broke!", self.flag);
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
            wooden_sword: WeaponData::new(10, 100, Weapon::Wooden),
            bronze_sword: WeaponData::new(20, 150, Weapon::Bronze),
            iron_sword: WeaponData::new(50, 200, Weapon::Iron),
            steel_sword: WeaponData::new(200, 500, Weapon::Steel),
            mystic_sword: WeaponData::new(500, 1_000, Weapon::Mystic),
            wizard_staff: WeaponData::new(1_000, 2_000, Weapon::WizardStaff),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        csv_table(vec![
            "Weapon,Owned,Equipped,Damage,Durability".to_string(),
            self.wooden_sword.to_string(),
            self.bronze_sword.to_string(),
            self.iron_sword.to_string(),
            self.steel_sword.to_string(),
            self.mystic_sword.to_string(),
            self.wizard_staff.to_string(),
        ])
    }

    pub fn get(&mut self, item_flag: &Weapon) -> &mut WeaponData {
        match item_flag {
            Weapon::Bronze => &mut self.bronze_sword,
            Weapon::Iron => &mut self.iron_sword,
            Weapon::Mystic => &mut self.mystic_sword,
            Weapon::Steel => &mut self.steel_sword,
            Weapon::WizardStaff => &mut self.wizard_staff,
            Weapon::Wooden => &mut self.wooden_sword,
        }
    }

    /// For use in developer mode only
    pub fn toggle_own(&mut self, item_flag: &Weapon) {
        let item = self.get(item_flag);

        item.owns = !item.owns;
    }
}

// -------------------------------------------------- Economy -------------------------------------------------- //

impl WeaponsInventory {
    fn shop() -> BTreeMap<Weapon, usize> {
        BTreeMap::from([
            (Weapon::Wooden, 10),
            (Weapon::Bronze, 50),
            (Weapon::Iron, 100),
            (Weapon::Steel, 500),
            (Weapon::Mystic, 1_000),
            (Weapon::WizardStaff, 10_000),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owned".to_string()];

        Self::shop().iter().for_each(|(flag, price)| {
            let owned = player.weapons.get(flag).owns;

            let string = format!("{},{},{}", flag, price, checkmark(owned));
            strings.push(string)
        });

        csv_table(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn select() -> Weapon {
        let shop = Self::shop();
        let items: Vec<String> = shop.keys().map(|flag| flag.to_string()).collect();

        let selector = select(&items, None);
        let selected_item = items
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        shop.iter()
            .find(|item| item.0.to_string() == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
            .clone()
    }

    pub fn buy(player: &mut Player, weapon: Weapon, payment: bool) -> Result<(), InventoryError> {
        let shop = Self::shop();
        let price = shop.get(&weapon).ok_or(InventoryError::ItemNotExist)?;

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;

            if gold < *price {
                return Err(InventoryError::NotEnoughGold);
            }

            *wallet -= *price;
        }

        let owns_item = &mut player.weapons.get(&weapon).owns;

        if *owns_item {
            return Err(InventoryError::ItemOwned);
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, weapon: Weapon, payment: bool) -> Result<(), InventoryError> {
        let shop: BTreeMap<Weapon, usize> = Self::shop();
        let price: &usize = shop.get(&weapon).ok_or(InventoryError::ItemNotExist)?;
        let owns_item = &mut player.weapons.get(&weapon).owns;

        if !*owns_item {
            return Err(InventoryError::ItemNotOwned);
        }

        *owns_item = false;

        if payment {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = price / 2;

            *wallet += price;
        }

        Ok(())
    }
}
