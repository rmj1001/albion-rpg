use csv_to_table as table;
use serde::{Deserialize, Serialize};

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
    pub fn print(&self) {
        let inv = &self;

        let inventory: String = vec![
            format!("Item,Quantity,Buy Price,Sale Price"),
            format!(
                "Bait,{},{},{}",
                inv.bait.quantity,
                inv.bait.price,
                inv.bait.price / 2
            ),
            format!(
                "Seeds,{},{},{}",
                inv.seeds.quantity,
                inv.seeds.price,
                inv.seeds.price / 2
            ),
            format!(
                "Furs,{},{},{}",
                inv.furs.quantity,
                inv.furs.price,
                inv.furs.price / 2
            ),
            format!(
                "Fish,{},{},{}",
                inv.fish.quantity,
                inv.fish.price,
                inv.fish.price / 2
            ),
            format!(
                "Food,{},{},{}",
                inv.food.quantity,
                inv.food.price,
                inv.food.price / 2
            ),
            format!(
                "Wood,{},{},{}",
                inv.wood.quantity,
                inv.wood.price,
                inv.wood.price / 2
            ),
            format!(
                "Ore,{},{},{}",
                inv.ore.quantity,
                inv.ore.price,
                inv.ore.price / 2
            ),
            format!(
                "Ingots,{},{},{}",
                inv.ingots.quantity,
                inv.ingots.price,
                inv.ingots.price / 2
            ),
            format!(
                "Potions,{},{},{}",
                inv.potions.quantity,
                inv.potions.price,
                inv.potions.price / 2
            ),
            format!(
                "Rubies,{},{},{}",
                inv.rubies.quantity,
                inv.rubies.price,
                inv.rubies.price / 2
            ),
            format!(
                "Magic Scrolls,{},{},{}",
                inv.magic_scrolls.quantity,
                inv.magic_scrolls.price,
                inv.magic_scrolls.price / 2
            ),
            format!(
                "Bones,{},{},{}",
                inv.bones.quantity,
                inv.bones.price,
                inv.bones.price / 2
            ),
            format!(
                "Dragon Hides,{},{},{}",
                inv.dragon_hides.quantity,
                inv.dragon_hides.price,
                inv.dragon_hides.price / 2
            ),
            format!(
                "Runic Tablets,{},{},{}",
                inv.runic_tablets.quantity,
                inv.runic_tablets.price,
                inv.runic_tablets.price / 2
            ),
        ]
        .join("\n");

        let table = table::iter::from_reader(inventory.as_bytes()).to_string();

        println!("{table}\n");
    }
}
