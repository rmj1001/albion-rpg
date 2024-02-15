use serde::{Deserialize, Serialize};

use crate::utils::{math::Operation, messages::*, tui::table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub price: usize,
    pub quantity: usize,
}

impl Item {
    pub fn new(name: &str, price: usize) -> Self {
        Self {
            name: name.to_string(),
            price,
            quantity: 0,
        }
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn new() -> MundaneInventory {
        MundaneInventory {
            bait: Item::new("Bait", 1),
            seeds: Item::new("Seeds", 1),
            furs: Item::new("Fur", 5),
            fish: Item::new("Fish", 10),
            food: Item::new("Food", 25),
            wood: Item::new("Wood", 20),
            ore: Item::new("Ore", 30),
            ingots: Item::new("Ingot", 50),
            potions: Item::new("Potion", 20),
            rubies: Item::new("Ruby", 200),
            magic_scrolls: Item::new("Magic Scroll", 50),
            bones: Item::new("Bone", 50),
            dragon_hides: Item::new("Dragon Hide", 200),
            runic_tablets: Item::new("Runic Tablet", 1000),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        let inv = &self;

        fn entry(item: &Item) -> String {
            format!("{},{},{},{}", item.name, item.quantity, item.price, item.price / 2)
        }

        table_from_csv(vec![
            "Item,Quantity,Buy Price,Sale Price".to_string(),
            entry(&inv.bait),
            entry(&inv.seeds),
            entry(&inv.furs),
            entry(&inv.fish),
            entry(&inv.food),
            entry(&inv.wood),
            entry(&inv.ore),
            entry(&inv.ingots),
            entry(&inv.potions),
            entry(&inv.rubies),
            entry(&inv.magic_scrolls),
            entry(&inv.bones),
            entry(&inv.dragon_hides),
            entry(&inv.runic_tablets),
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

    pub fn arithmetic(&mut self, item_flag: InventoryItemFlag, operation: Operation<usize>) -> Result<(), &str> {
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
                Err("")
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
