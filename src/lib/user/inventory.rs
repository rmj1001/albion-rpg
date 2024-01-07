use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub enum ItemNames {
    Fish,
    CookedFish,
    Wood,
    Ore,
    Ingots,
}

#[derive(Serialize, Deserialize)]
pub struct MundaneInventory {
    pub fish: Item,
    pub cooked_fish: Item,
    pub wood: Item,
    pub ore: Item,
    pub ingots: Item,
}
