use serde::{Deserialize, Serialize};

use crate::lib::{math::Operation, messages::*, tui::print_table};

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

pub enum InventoryItemFlag {
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
    InvalidItem,
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

    pub fn retrieve_item(&mut self, item_flag: &InventoryItemFlag) -> Option<&mut Item> {
        match item_flag {
            InventoryItemFlag::Bait => Some(&mut self.bait),
            InventoryItemFlag::Bones => Some(&mut self.bones),
            InventoryItemFlag::DragonHides => Some(&mut self.dragon_hides),
            InventoryItemFlag::Fish => Some(&mut self.fish),
            InventoryItemFlag::Food => Some(&mut self.food),
            InventoryItemFlag::Furs => Some(&mut self.furs),
            InventoryItemFlag::Ingots => Some(&mut self.ingots),
            InventoryItemFlag::MagicScrolls => Some(&mut self.ingots),
            InventoryItemFlag::Ore => Some(&mut self.ore),
            InventoryItemFlag::Potions => Some(&mut self.potions),
            InventoryItemFlag::Rubies => Some(&mut self.rubies),
            InventoryItemFlag::RunicTablets => Some(&mut self.runic_tablets),
            InventoryItemFlag::Seeds => Some(&mut self.seeds),
            InventoryItemFlag::Wood => Some(&mut self.wood),
            InventoryItemFlag::InvalidItem => None,
        }
    }

    pub fn arithmetic(
        &mut self,
        item_flag: InventoryItemFlag,
        operation: Operation<usize>,
    ) -> Result<(), &str> {
        let item_result = self.retrieve_item(&item_flag);

        if item_result.is_none() {
            return Err("The InventoryItemFlag passed was the Invalid variant.");
        }

        let item = item_result.unwrap();

        match operation {
            Operation::Add(amount) => {
                item.quantity += amount;
                Ok(())
            }

            Operation::Subtract(amount) => {
                if amount > item.quantity {
                    Err("The quantity is too small to subtract that amount.")
                } else {
                    item.quantity -= amount;
                    Ok(())
                }
            }

            Operation::Multiply(amount) => {
                item.quantity *= amount;
                Ok(())
            }

            Operation::Divide(amount) => {
                item.quantity /= amount;
                Ok(())
            }
            Operation::Cancel => {
                cancelling();
                Ok(())
            }
            Operation::Invalid => {
                failure("Invalid Operator");
                Err("Operation failed: Invalid Operator")
            }
        }
    }

    pub fn purchase(
        &mut self,
        wallet: &mut usize,
        item_flag: &InventoryItemFlag,
        amount: usize,
        deduct_wallet: bool,
    ) -> Result<(), String> {
        let item_result = self.retrieve_item(item_flag);

        if item_result.is_none() {
            return Err("The InventoryItemFlag passed was the Invalid variant.".to_string());
        }

        let item = item_result.unwrap();
        let price = amount * item.price;

        if deduct_wallet && price > *wallet {
            return Err(format!(
                "You do not have enough gold to purchase {} {}.",
                amount, item.name
            ));
        }

        item.quantity += amount;

        if deduct_wallet {
            *wallet -= price;
        }

        Ok(())
    }

    pub fn sell(
        &mut self,
        wallet: &mut usize,
        item_flag: &InventoryItemFlag,
        amount: usize,
        add_to_wallet: bool,
    ) -> Result<(), String> {
        let item_result = self.retrieve_item(item_flag);

        if item_result.is_none() {
            return Err("The InventoryItemFlag passed was the Invalid variant.".to_string());
        }

        let item = item_result.unwrap();
        let price = amount * (item.price / 2);

        if amount > item.quantity {
            return Err(format!("You do not have {} {}.", amount, item.name));
        }

        item.quantity -= amount;

        if add_to_wallet {
            *wallet += price;
        }

        Ok(())
    }
}
