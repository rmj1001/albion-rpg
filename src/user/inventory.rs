use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GuildItemNames {
    Fish,
    CookedFish,
    Wood,
    Ore,
    Ingots,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MundaneInventory {
    pub bait: Item,
    pub seeds: Item,
    pub furs: Item,
    pub fish: Item,
    pub food: Item,
    pub wood: Item,
    pub ore: Item,
    pub ingots: Item,
    pub potions: Item,
    pub rubies: Item,
    pub magic_scrolls: Item,
    pub bones: Item,
    pub dragon_hides: Item,
    pub runic_tablets: Item,
}
