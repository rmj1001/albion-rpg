use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;

use crate::{
    data::player::Player,
    utils::{
        input::{input_generic, select_from_vector},
        tui::table_from_csv,
    },
    InventoryError,
};

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

impl Item {
    pub fn name(&self) -> &'static str {
        match self {
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
        }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        ItemInventory {
            bait: 0,
            seeds: 0,
            furs: 0,
            fish: 0,
            food: 0,
            wood: 0,
            ore: 0,
            ingots: 0,
            potions: 0,
            rubies: 0,
            magic_scrolls: 0,
            bones: 0,
            dragon_hides: 0,
            runic_tablets: 0,
        }
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

impl ItemInventory {
    pub fn shop() -> BTreeMap<Item, usize> {
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

    pub fn print_shop(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,usize,Quantity,,Item,usize,Quantity".to_string()];

        type ShopItem = (Item, usize, usize);
        type Pair = (ShopItem, ShopItem);

        let mut pairs: Vec<Pair> = vec![];
        let mut current_pair: Pair = ((Item::Bait, 0, 0), (Item::Bait, 0, 0));
        let mut index: usize = 0;

        for (flag, usize) in Self::shop() {
            let quantity = player.items.get(flag);

            if index == 0 {
                current_pair.0 = (flag, usize, *quantity);
                index += 1;
            } else if index == 1 {
                current_pair.1 = (flag, usize, *quantity);
                pairs.push(current_pair);
                index = 0;
            }
        }

        for (item1, item2) in pairs {
            strings.push(format!(
                "{},{},{},,{},{},{}",
                item1.0.name(),
                item1.1,
                item1.2,
                item2.0.name(),
                item2.1,
                item2.2
            ));
        }

        table_from_csv(strings);
    }

    pub fn build_transaction() -> crate::Result<(Item, usize)> {
        let item = Self::picker();
        let quantity_result = input_generic::<usize>("Quantity:");

        match quantity_result {
            Ok(quantity) => Ok((item, quantity)),
            Err(error) => Err(error),
        }
    }

    pub fn picker() -> Item {
        let shop = Self::shop();
        let items = shop.keys();
        let item_names: Vec<String> = items.map(|item| item.name().to_string()).collect();

        let selector = select_from_vector(item_names.clone(), None);
        let selected_item = item_names
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        *Self::shop()
            .iter()
            .find(|item| item.0.name() == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
    }

    pub fn buy(player: &mut Player, flag: Item, quantity: usize, use_wallet: bool) -> crate::Result<()> {
        let shop = Self::shop();
        let usize: usize = *shop.get(&flag).expect("Item not found in hashmap.");

        if use_wallet {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let usize = quantity * usize;

            if gold < usize {
                return Err(InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= usize;
        }

        let item = player.items.get(flag);

        *item += quantity;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: Item, quantity: usize, use_wallet: bool) -> crate::Result<()> {
        let shop = Self::shop();
        let usize: usize = *shop.get(&flag).expect("Item not found in hashmap.");
        let item = player.items.get(flag);

        if *item == 0 || *item < quantity {
            return Err(InventoryError::NotEnoughItem(flag.name().to_string()).boxed());
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
