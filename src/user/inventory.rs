use serde::{Deserialize, Serialize};

use crate::lib::{
    math::Operation,
    tui::{press_enter_to_continue, print_table},
};

use super::profile::UserProfile;

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

    pub fn retrieve_item(&mut self, item_flag: InventoryItemFlag) -> &mut Item {
        match item_flag {
            InventoryItemFlag::Bait => &mut self.bait,
            InventoryItemFlag::Bones => &mut self.bones,
            InventoryItemFlag::DragonHides => &mut self.dragon_hides,
            InventoryItemFlag::Fish => &mut self.fish,
            InventoryItemFlag::Food => &mut self.food,
            InventoryItemFlag::Furs => &mut self.furs,
            InventoryItemFlag::Ingots => &mut self.ingots,
            InventoryItemFlag::MagicScrolls => &mut self.ingots,
            InventoryItemFlag::Ore => &mut self.ore,
            InventoryItemFlag::Potions => &mut self.potions,
            InventoryItemFlag::Rubies => &mut self.rubies,
            InventoryItemFlag::RunicTablets => &mut self.runic_tablets,
            InventoryItemFlag::Seeds => &mut self.seeds,
            InventoryItemFlag::Wood => &mut self.wood,
        }
    }

    pub fn arithmetic(
        &mut self,
        item_flag: InventoryItemFlag,
        operation: Operation<usize>,
    ) -> Result<(), &str> {
        let item = self.retrieve_item(item_flag);

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
                println!("\nCancelling.");
                press_enter_to_continue();
                Ok(())
            }
            Operation::Invalid => {
                println!("\nOperation failed: Invalid Operator");
                press_enter_to_continue();
                Err("Operation failed: Invalid Operator")
            }
        }
    }

    pub fn purchase(
        &mut self,
        user: &mut UserProfile,
        item_flag: InventoryItemFlag,
        amount: usize,
    ) -> Result<(), String> {
        let item = self.retrieve_item(item_flag);
        let price = amount * item.price;

        if price > user.bank.wallet {
            return Err(format!(
                "The price is too high to purchase {} {}.",
                amount, item.name
            ));
        }

        item.quantity += amount;
        user.bank.wallet -= price;
        Ok(())
    }

    pub fn sell(
        &mut self,
        user: &mut UserProfile,
        item_flag: InventoryItemFlag,
        amount: usize,
    ) -> Result<(), String> {
        let item = self.retrieve_item(item_flag);
        let price = amount * (item.price / 2);

        if amount > item.quantity {
            return Err(format!("You do not have {} {}.", amount, item.name));
        }

        item.quantity -= amount;
        user.bank.wallet += price;
        Ok(())
    }
}
