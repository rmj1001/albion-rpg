use crate::{
    data::player::Player,
    prelude::{checkmark, csv_table, error, select},
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, result::Result};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Types {
    Leather,
    Bronze,
    Iron,
    Steel,
    Dragonhide,
    Mystic,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?} Armor")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    pub owns: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub equipped: bool,
    pub flag: Types,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            owns: false,
            equipped: false,
            defense: 5,
            durability: 50,
            default_durability: 50,
            flag: Types::Leather,
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
            self.defense,
            self.durability,
        )
    }
}

impl Data {
    pub fn new(defense: usize, durability: usize, flag: Types) -> Self {
        Self {
            defense,
            durability,
            flag,
            ..Default::default()
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
        println!("Your {} broke!", self.flag);
        self.owns = false;
        self.durability = self.default_durability;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Inventory {
    pub leather: Data,
    pub bronze: Data,
    pub iron: Data,
    pub steel: Data,
    pub dragonhide: Data,
    pub mystic: Data,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            leather: Data::new(10, 100, Types::Leather),
            bronze: Data::new(30, 200, Types::Bronze),
            iron: Data::new(50, 300, Types::Iron),
            steel: Data::new(100, 500, Types::Steel),
            dragonhide: Data::new(200, 500, Types::Dragonhide),
            mystic: Data::new(1_000, 10_000, Types::Mystic),
        }
    }
}

impl Inventory {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn table(&self) {
        csv_table(&[
            "Armor,Owned,Equipped,Defense,Durability".to_string(),
            self.leather.to_string(),
            self.bronze.to_string(),
            self.iron.to_string(),
            self.steel.to_string(),
            self.dragonhide.to_string(),
            self.mystic.to_string(),
        ]);
    }

    pub fn get(&mut self, item_flag: &Types) -> &mut Data {
        match item_flag {
            Types::Bronze => &mut self.bronze,
            Types::Dragonhide => &mut self.dragonhide,
            Types::Iron => &mut self.iron,
            Types::Leather => &mut self.leather,
            Types::Mystic => &mut self.mystic,
            Types::Steel => &mut self.steel,
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
            (Types::Leather, 100),
            (Types::Bronze, 300),
            (Types::Iron, 1_000),
            (Types::Steel, 5_000),
            (Types::Dragonhide, 10_000),
            (Types::Mystic, 20_000),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owns".to_string()];

        for (flag, price) in &Self::shop() {
            let string = format!("{},{},{}", flag, price, checkmark(player.armor.get(flag).owns));
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

    pub fn buy(player: &mut Player, flag: &Types, payment: bool) -> Result<(), error::Inventory> {
        let shop = Self::shop();
        let price: &usize = shop.get(flag).ok_or(error::Inventory::TransactionFailed)?;

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;

            if gold < *price {
                return Err(error::Inventory::NotEnoughGold);
            }

            *wallet -= *price;
        }

        let owns_item = &mut player.armor.get(flag).owns;

        if *owns_item {
            return Err(error::Inventory::ItemOwned);
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: &Types, payment: bool) -> Result<(), error::Inventory> {
        let shop: BTreeMap<Types, usize> = Self::shop();
        let price: &usize = shop.get(flag).ok_or(error::Inventory::TransactionFailed)?;
        let owns_item: &mut bool = &mut player.armor.get(flag).owns;

        if !*owns_item {
            return Err(error::Inventory::ItemNotOwned);
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
