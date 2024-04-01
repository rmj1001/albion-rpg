use crate::utils::tui::{checkmark, csv_table};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

use crate::data::player::Player;
use crate::prelude::{select, InventoryError};
use std::result::Result;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Types {
    Wooden,
    Bronze,
    Iron,
    Steel,
    Mystic,
    WizardStaff,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::WizardStaff => "Wizard Staff".to_string(),
            sword => format!("{sword:?} Sword"),
        };

        write!(f, "{string}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    pub owns: bool,
    pub equipped: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub flag: Types,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            owns: false,
            equipped: false,
            damage: 5,
            durability: 50,
            default_durability: 50,
            flag: Types::Wooden,
        }
    }
}

impl Display for Data {
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

impl Data {
    pub fn new(damage: usize, durability: usize, flag: Types) -> Self {
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
pub struct Inventory {
    pub wooden_sword: Data,
    pub bronze_sword: Data,
    pub iron_sword: Data,
    pub steel_sword: Data,
    pub mystic_sword: Data,
    pub wizard_staff: Data,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            wooden_sword: Data::new(10, 100, Types::Wooden),
            bronze_sword: Data::new(20, 150, Types::Bronze),
            iron_sword: Data::new(50, 200, Types::Iron),
            steel_sword: Data::new(200, 500, Types::Steel),
            mystic_sword: Data::new(500, 1_000, Types::Mystic),
            wizard_staff: Data::new(1_000, 2_000, Types::WizardStaff),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        csv_table(&[
            "Weapon,Owned,Equipped,Damage,Durability".to_string(),
            self.wooden_sword.to_string(),
            self.bronze_sword.to_string(),
            self.iron_sword.to_string(),
            self.steel_sword.to_string(),
            self.mystic_sword.to_string(),
            self.wizard_staff.to_string(),
        ]);
    }

    pub fn get(&mut self, item_flag: &Types) -> &mut Data {
        match item_flag {
            Types::Bronze => &mut self.bronze_sword,
            Types::Iron => &mut self.iron_sword,
            Types::Mystic => &mut self.mystic_sword,
            Types::Steel => &mut self.steel_sword,
            Types::WizardStaff => &mut self.wizard_staff,
            Types::Wooden => &mut self.wooden_sword,
        }
    }

    /// For use in developer mode only
    pub fn toggle_own(&mut self, item_flag: &Types) {
        let item = self.get(item_flag);

        item.owns = !item.owns;
    }
}

// -------------------------------------------------- Economy -------------------------------------------------- //

impl Inventory {
    fn shop() -> BTreeMap<Types, usize> {
        BTreeMap::from([
            (Types::Wooden, 10),
            (Types::Bronze, 50),
            (Types::Iron, 100),
            (Types::Steel, 500),
            (Types::Mystic, 1_000),
            (Types::WizardStaff, 10_000),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owned".to_string()];

        for (flag, price) in &Self::shop() {
            let owned = player.weapons.get(flag).owns;

            let string = format!("{},{},{}", flag, price, checkmark(owned));
            strings.push(string);
        }

        csv_table(&strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn select() -> Types {
        let shop = Self::shop();
        let items: Vec<String> = shop.keys().map(std::string::ToString::to_string).collect();

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

    pub fn buy(player: &mut Player, weapon: &Types, payment: bool) -> Result<(), InventoryError> {
        let shop = Self::shop();
        let price = shop.get(weapon).ok_or(InventoryError::ItemNotExist)?;

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;

            if gold < *price {
                return Err(InventoryError::NotEnoughGold);
            }

            *wallet -= *price;
        }

        let owns_item = &mut player.weapons.get(weapon).owns;

        if *owns_item {
            return Err(InventoryError::ItemOwned);
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, weapon: &Types, payment: bool) -> Result<(), InventoryError> {
        let shop: BTreeMap<Types, usize> = Self::shop();
        let price: &usize = shop.get(weapon).ok_or(InventoryError::ItemNotExist)?;
        let owns_item = &mut player.weapons.get(weapon).owns;

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
