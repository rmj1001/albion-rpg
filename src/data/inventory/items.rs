use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{collections::BTreeMap, fmt::Display};

use crate::data::player::Player;
use crate::prelude::{csv_table, input_generic, select, InventoryError};
use std::result::Result;

type ShopItem = (Item, usize, usize);
type Pair = (ShopItem, ShopItem);

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Item {
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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: &str = match self {
            Item::Bait => "Bait",
            Item::Seeds => "Seeds",
            Item::Furs => "Fur",
            Item::Fish => "Fish",
            Item::Food => "Food",
            Item::Wood => "Wood",
            Item::Ore => "Ore",
            Item::Ingots => "Ingot",
            Item::Potions => "Potion",
            Item::Rubies => "Ruby",
            Item::MagicScrolls => "Magic Scroll",
            Item::Bones => "Bone",
            Item::DragonHides => "Dragon Hide",
            Item::RunicTablets => "Runic Tablet",
        };

        write!(f, "{string}")
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GuildItem {
    Bait,
    Fish,
    Food,
    Wood,
    Ore,
    Ingots,
    Gold,
}

impl GuildItem {
    pub fn to_mundane_item(&self) -> Option<Item> {
        match self {
            GuildItem::Ore => Some(Item::Ore),
            GuildItem::Bait => Some(Item::Bait),
            GuildItem::Fish => Some(Item::Fish),
            GuildItem::Food => Some(Item::Food),
            GuildItem::Ingots => Some(Item::Ingots),
            GuildItem::Wood => Some(Item::Wood),
            GuildItem::Gold => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ItemInventory {
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

impl ItemInventory {
    pub fn new() -> ItemInventory {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn get(&mut self, flag: Item) -> &mut usize {
        match flag {
            Item::Bait => &mut self.bait,
            Item::Bones => &mut self.bones,
            Item::DragonHides => &mut self.dragon_hides,
            Item::Fish => &mut self.fish,
            Item::Food => &mut self.food,
            Item::Furs => &mut self.furs,
            Item::Ingots => &mut self.ingots,
            Item::MagicScrolls => &mut self.magic_scrolls,
            Item::Ore => &mut self.ore,
            Item::Potions => &mut self.potions,
            Item::Rubies => &mut self.rubies,
            Item::RunicTablets => &mut self.runic_tablets,
            Item::Seeds => &mut self.seeds,
            Item::Wood => &mut self.wood,
        }
    }
}

// -------------------------------------------------- Economy -------------------------------------------------- //

impl ItemInventory {
    fn shop() -> BTreeMap<Item, usize> {
        BTreeMap::from([
            (Item::Bait, 1),
            (Item::Seeds, 1),
            (Item::Furs, 50),
            (Item::Fish, 5),
            (Item::Food, 10),
            (Item::Wood, 10),
            (Item::Ore, 15),
            (Item::Ingots, 30),
            (Item::Potions, 20),
            (Item::Rubies, 100),
            (Item::MagicScrolls, 200),
            (Item::Bones, 10),
            (Item::DragonHides, 50),
            (Item::RunicTablets, 300),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,usize,Quantity,,Item,usize,Quantity".to_string()];

        let mut pairs: Vec<Pair> = vec![];
        let mut current_pair: Pair = ((Item::Bait, 0, 0), (Item::Bait, 0, 0));
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

    pub fn build_transaction() -> Result<(Item, usize), InventoryError> {
        let item = Self::select();

        match input_generic::<usize>("Quantity:") {
            Ok(quantity) => Ok((item, quantity)),
            Err(_) => Err(InventoryError::TransactionFailed),
        }
    }

    pub fn select() -> Item {
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

    pub fn buy(player: &mut Player, flag: Item, quantity: usize, use_wallet: bool) -> Result<(), InventoryError> {
        let shop = Self::shop();
        let usize: usize = *shop.get(&flag).ok_or(InventoryError::TransactionFailed)?;

        if use_wallet {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let usize = quantity * usize;

            if gold < usize {
                return Err(InventoryError::NotEnoughGold);
            }

            *wallet -= usize;
        }

        let item = player.items.get(flag);

        *item += quantity;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: Item, quantity: usize, use_wallet: bool) -> Result<(), InventoryError> {
        let shop = Self::shop();
        let usize: usize = *shop.get(&flag).ok_or(InventoryError::ItemNotExist)?;
        let item = player.items.get(flag);

        if *item == 0 || *item < quantity {
            return Err(InventoryError::NotEnoughItem(flag.to_string()));
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
