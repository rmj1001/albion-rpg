use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{collections::BTreeMap, fmt::Display};

use crate::data::player::Player;
use crate::prelude::{csv_table, error, generic_prompt, select};
use std::result::Result;

type ShopItem = (Types, usize, usize);
type Pair = (ShopItem, ShopItem);

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Types {
    Bait,
    Seeds,
    Furs,
    Fish,
    Food,
    Wood,
    Ore,
    Ingots,
    Potions,
    Rubies,
    MagicScrolls,
    Bones,
    DragonHides,
    RunicTablets,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: &str = match self {
            Types::Bait => "Bait",
            Types::Seeds => "Seeds",
            Types::Furs => "Fur",
            Types::Fish => "Fish",
            Types::Food => "Food",
            Types::Wood => "Wood",
            Types::Ore => "Ore",
            Types::Ingots => "Ingot",
            Types::Potions => "Potion",
            Types::Rubies => "Ruby",
            Types::MagicScrolls => "Magic Scroll",
            Types::Bones => "Bone",
            Types::DragonHides => "Dragon Hide",
            Types::RunicTablets => "Runic Tablet",
        };

        write!(f, "{string}")
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GuildTypes {
    Bait,
    Fish,
    Food,
    Wood,
    Ore,
    Ingots,
    Gold,
}

impl GuildTypes {
    pub fn to_mundane_item(&self) -> Option<Types> {
        match self {
            GuildTypes::Ore => Some(Types::Ore),
            GuildTypes::Bait => Some(Types::Bait),
            GuildTypes::Fish => Some(Types::Fish),
            GuildTypes::Food => Some(Types::Food),
            GuildTypes::Ingots => Some(Types::Ingots),
            GuildTypes::Wood => Some(Types::Wood),
            GuildTypes::Gold => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Inventory {
    pub bait: usize,
    pub seeds: usize,
    pub furs: usize,
    pub fish: usize,
    pub food: usize,
    pub wood: usize,
    pub ore: usize,
    pub ingots: usize,
    pub potions: usize,
    pub rubies: usize,
    pub magic_scrolls: usize,
    pub bones: usize,
    pub dragon_hides: usize,
    pub runic_tablets: usize,
}

impl Inventory {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn get(&mut self, flag: Types) -> &mut usize {
        match flag {
            Types::Bait => &mut self.bait,
            Types::Bones => &mut self.bones,
            Types::DragonHides => &mut self.dragon_hides,
            Types::Fish => &mut self.fish,
            Types::Food => &mut self.food,
            Types::Furs => &mut self.furs,
            Types::Ingots => &mut self.ingots,
            Types::MagicScrolls => &mut self.magic_scrolls,
            Types::Ore => &mut self.ore,
            Types::Potions => &mut self.potions,
            Types::Rubies => &mut self.rubies,
            Types::RunicTablets => &mut self.runic_tablets,
            Types::Seeds => &mut self.seeds,
            Types::Wood => &mut self.wood,
        }
    }
}

// -------------------------------------------------- Economy -------------------------------------------------- //

impl Inventory {
    fn shop() -> BTreeMap<Types, usize> {
        BTreeMap::from([
            (Types::Bait, 1),
            (Types::Seeds, 1),
            (Types::Furs, 50),
            (Types::Fish, 5),
            (Types::Food, 10),
            (Types::Wood, 10),
            (Types::Ore, 15),
            (Types::Ingots, 30),
            (Types::Potions, 20),
            (Types::Rubies, 100),
            (Types::MagicScrolls, 200),
            (Types::Bones, 10),
            (Types::DragonHides, 50),
            (Types::RunicTablets, 300),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Quantity,,Item,Price,Quantity".to_string()];

        let mut pairs: Vec<Pair> = vec![];
        let mut current_pair: Pair = ((Types::Bait, 0, 0), (Types::Bait, 0, 0));
        let mut index: usize = 0;

        for (flag, usize) in &Self::shop() {
            let quantity = player.items.get(*flag);

            if index == 0 {
                current_pair.0 = (*flag, *usize, *quantity);
                index += 1;
            } else if index == 1 {
                current_pair.1 = (*flag, *usize, *quantity);
                pairs.push(current_pair);
                index = 0;
            }
        }

        for (item1, item2) in &pairs {
            strings.push(format!(
                "{},{},{},,{},{},{}",
                item1.0, item1.1, item1.2, item2.0, item2.1, item2.2
            ));
        }

        csv_table(&strings);
    }

    pub fn build_transaction() -> Result<(Types, usize), error::Inventory> {
        let item = Self::select();

        match generic_prompt::<usize>("Quantity:") {
            Ok(quantity) => Ok((item, quantity)),
            Err(_) => Err(error::Inventory::TransactionFailed),
        }
    }

    pub fn select() -> Types {
        let shop = Self::shop();
        let items = shop.keys();
        let item_names: Vec<String> = items.map(std::string::ToString::to_string).collect();

        let selector = select(&item_names, None);
        let selected_item = item_names
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        *Self::shop()
            .iter()
            .find(|item| item.0.to_string() == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
    }

    pub fn buy(player: &mut Player, flag: Types, quantity: usize, use_wallet: bool) -> Result<(), error::Inventory> {
        let shop = Self::shop();
        let usize: usize = *shop.get(&flag).ok_or(error::Inventory::TransactionFailed)?;

        if use_wallet {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let usize = quantity * usize;

            if gold < usize {
                return Err(error::Inventory::NotEnoughGold);
            }

            *wallet -= usize;
        }

        let item = player.items.get(flag);

        *item += quantity;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: Types, quantity: usize, use_wallet: bool) -> Result<(), error::Inventory> {
        let shop = Self::shop();
        let usize: usize = *shop.get(&flag).ok_or(error::Inventory::ItemNotExist)?;
        let item = player.items.get(flag);

        if *item == 0 || *item < quantity {
            return Err(error::Inventory::NotEnoughItem(flag.to_string()));
        }

        *item -= quantity;

        if use_wallet {
            let wallet: &mut usize = &mut player.bank.wallet;
            let usize: usize = quantity * (usize / 2);

            *wallet += usize;
        }

        Ok(())
    }
}
