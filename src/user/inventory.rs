use serde::{Deserialize, Serialize};

use crate::lib::tui::print_table;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub price: usize,
    pub quantity: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GuildItemNames {
    Fish,
    CookedFish,
    Wood,
    Ore,
    Ingots,
}

pub enum InventoryItem {
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

impl MundaneInventory {
    pub fn print_table(&self) {
        let inv = &self;

        print_table(vec![
            "Item,Quantity,Buy Price,Sale Price".to_string(),
            format!(
                "{},{},{},{}",
                inv.bait.name,
                inv.bait.quantity,
                inv.bait.price,
                inv.bait.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.seeds.name,
                inv.seeds.quantity,
                inv.seeds.price,
                inv.seeds.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.furs.name,
                inv.furs.quantity,
                inv.furs.price,
                inv.furs.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.fish.name,
                inv.fish.quantity,
                inv.fish.price,
                inv.fish.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.food.name,
                inv.food.quantity,
                inv.food.price,
                inv.food.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.wood.name,
                inv.wood.quantity,
                inv.wood.price,
                inv.wood.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.ore.name,
                inv.ore.quantity,
                inv.ore.price,
                inv.ore.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.ingots.name,
                inv.ingots.quantity,
                inv.ingots.price,
                inv.ingots.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.potions.name,
                inv.potions.quantity,
                inv.potions.price,
                inv.potions.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.rubies.name,
                inv.rubies.quantity,
                inv.rubies.price,
                inv.rubies.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.magic_scrolls.name,
                inv.magic_scrolls.quantity,
                inv.magic_scrolls.price,
                inv.magic_scrolls.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.bones.name,
                inv.bones.quantity,
                inv.bones.price,
                inv.bones.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.dragon_hides.name,
                inv.dragon_hides.quantity,
                inv.dragon_hides.price,
                inv.dragon_hides.price / 2
            ),
            format!(
                "{},{},{},{}",
                inv.runic_tablets.name,
                inv.runic_tablets.quantity,
                inv.runic_tablets.price,
                inv.runic_tablets.price / 2
            ),
        ])
    }
}
