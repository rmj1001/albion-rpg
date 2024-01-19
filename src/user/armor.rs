use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::lib::tui::{pretty_bool, print_table};

#[derive(Serialize, Deserialize, Debug)]
pub struct Armor {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub equipped: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Armor {
    pub fn decrease_durability(&mut self) {
        let random_damage = rand::thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability == 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmorInventory {
    pub leather: Armor,
    pub bronze: Armor,
    pub iron: Armor,
    pub steel: Armor,
    pub dragonhide: Armor,
    pub mystic: Armor,
}

pub enum ArmorItemFlag {
    Leather,
    Bronze,
    Iron,
    Steel,
    DragonHide,
    Mystic,
    Invalid,
}

impl ArmorInventory {
    pub fn print_table(&self) {
        print_table(vec![
            "Armor,Purchased,Equipped,Buy Price,Sale Price".to_string(),
            format!(
                "{},{},{},{},{}",
                self.leather.name,
                pretty_bool(self.leather.owns),
                pretty_bool(self.leather.equipped),
                self.leather.price,
                self.leather.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.bronze.name,
                pretty_bool(self.bronze.owns),
                pretty_bool(self.bronze.equipped),
                self.bronze.price,
                self.bronze.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.iron.name,
                pretty_bool(self.iron.owns),
                pretty_bool(self.iron.equipped),
                self.iron.price,
                self.iron.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.steel.name,
                pretty_bool(self.steel.owns),
                pretty_bool(self.steel.equipped),
                self.steel.price,
                self.steel.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.dragonhide.name,
                pretty_bool(self.dragonhide.owns),
                pretty_bool(self.dragonhide.equipped),
                self.dragonhide.price,
                self.dragonhide.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.mystic.name,
                pretty_bool(self.mystic.owns),
                pretty_bool(self.mystic.equipped),
                self.mystic.price,
                self.mystic.price / 2
            ),
        ])
    }

    pub fn retrieve_item(&mut self, item_flag: ArmorItemFlag) -> Option<&mut Armor> {
        match item_flag {
            ArmorItemFlag::Bronze => Some(&mut self.bronze),
            ArmorItemFlag::DragonHide => Some(&mut self.dragonhide),
            ArmorItemFlag::Iron => Some(&mut self.iron),
            ArmorItemFlag::Leather => Some(&mut self.leather),
            ArmorItemFlag::Mystic => Some(&mut self.mystic),
            ArmorItemFlag::Steel => Some(&mut self.steel),
            ArmorItemFlag::Invalid => None,
        }
    }

    /// For use in developer mode only
    pub fn own_item(&mut self, item_flag: ArmorItemFlag, flag: bool) {
        let item = self.retrieve_item(item_flag);

        if item.is_none() {
            return;
        }

        item.unwrap().owns = flag;
    }

    pub fn purchase(&mut self, wallet: &mut usize, item_flag: ArmorItemFlag) -> Result<(), String> {
        let item_option = self.retrieve_item(item_flag);

        if item_option.is_none() {
            return Err("The item was invalid.".to_string());
        }

        let item = item_option.unwrap();

        if item.price > *wallet {
            return Err(format!(
                "You do not have enough gold to purchase {}.",
                item.name
            ));
        }

        item.owns = true;
        *wallet -= item.price;
        Ok(())
    }

    pub fn sell(&mut self, wallet: &mut usize, item_flag: ArmorItemFlag) -> Result<(), String> {
        let item_option = self.retrieve_item(item_flag);

        if item_option.is_none() {
            return Err("The item was invalid.".to_string());
        }

        let item = item_option.unwrap();

        if !item.owns {
            return Err("You do not own this item.".to_string());
        }

        item.owns = false;
        *wallet += item.price / 2;
        Ok(())
    }
}
