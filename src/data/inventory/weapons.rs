use crate::utils::tui::{checkmark, csv_table};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{data::player::Player, prelude::*};

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

impl Default for WeaponData {
    fn default() -> Self {
        Self {
            owns: false,
            equipped: false,
            damage: 5,
            durability: 50,
            default_durability: 50,
            flag: Weapon::WoodenSword,
        }
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
        csv_table(vec![
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

// -------------------------------------------------- Economy -------------------------------------------------- //

impl WeaponsInventory {
    fn shop() -> BTreeMap<Weapon, usize> {
        BTreeMap::from([
            (Weapon::WoodenSword, 10),
            (Weapon::BronzeSword, 50),
            (Weapon::IronSword, 100),
            (Weapon::SteelSword, 500),
            (Weapon::MysticSword, 1_000),
            (Weapon::WizardStaff, 10_000),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owned".to_string()];

        Self::shop().iter().for_each(|(flag, price)| {
            let owned = player.weapons.get(flag).owns;

            let string = format!("{},{},{}", flag.name(), price, checkmark(owned));
            strings.push(string)
        });

        csv_table(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn select() -> Weapon {
        let shop = Self::shop();
        let items: Vec<String> = shop.keys().map(|flag| flag.name().to_string()).collect();

        let selector = select_from_vector(items.clone(), None);
        let selected_item = items
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        Self::shop()
            .iter()
            .find(|item| item.0.name() == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
            .clone()
    }

    pub fn buy(player: &mut Player, weapon: Weapon, payment: bool) -> Result<()> {
        let shop = Self::shop();
        let price = shop.get(&weapon).expect("Item not found in hashmap.");

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;

            if gold < *price {
                return Err(InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= *price;
        }

        let owns_item = &mut player.weapons.get(&weapon).owns;

        if *owns_item {
            return Err(InventoryError::ItemOwned.boxed());
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, weapon: Weapon, payment: bool) -> Result<()> {
        let shop: BTreeMap<Weapon, usize> = Self::shop();
        let price: &usize = shop.get(&weapon).expect("Item not found in hashmap.");
        let owns_item = &mut player.weapons.get(&weapon).owns;

        if !*owns_item {
            return Err(InventoryError::ItemNotOwned.boxed());
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
