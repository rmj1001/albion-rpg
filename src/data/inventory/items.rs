use serde::{Deserialize, Serialize};
use std::hash::Hash;

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
