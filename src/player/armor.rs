use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{pretty_bool, print_table};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Armor {
    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);

        if self.durability < random_damage {
            self.break_armor();
        } else {
            self.durability -= random_damage;
        }
    }

    pub fn break_armor(&mut self) {
        println!("Your {} broke!", self.name);
        self.owns = false;
        self.durability = self.default_durability;
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
    InvalidItem,
}

impl ArmorInventory {
    pub fn print_table(&self) {
        print_table(vec![
            "Armor,Purchased,Buy Price,Sale Price".to_string(),
            format!(
                "{},{},{},{}",
                self.leather.name,
                pretty_bool(self.leather.owns),
                self.leather.price,
                self.leather.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.bronze.name,
                pretty_bool(self.bronze.owns),
                self.bronze.price,
                self.bronze.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.iron.name,
                pretty_bool(self.iron.owns),
                self.iron.price,
                self.iron.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.steel.name,
                pretty_bool(self.steel.owns),
                self.steel.price,
                self.steel.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.dragonhide.name,
                pretty_bool(self.dragonhide.owns),
                self.dragonhide.price,
                self.dragonhide.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.mystic.name,
                pretty_bool(self.mystic.owns),
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
            ArmorItemFlag::InvalidItem => None,
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

    pub fn purchase(
        &mut self,
        wallet: &mut usize,
        item_flag: ArmorItemFlag,
        deduct_wallet: bool,
    ) -> Result<(), String> {
        let item_option = self.retrieve_item(item_flag);

        if item_option.is_none() {
            return Err("The item was invalid.".to_string());
        }

        let item = item_option.unwrap();

        if item.owns {
            return Err(format!("You already own {}.", item.name));
        }

        if deduct_wallet && item.price > *wallet {
            return Err(format!("You do not have enough gold to purchase {}.", item.name));
        }

        if deduct_wallet {
            *wallet -= item.price;
        }

        item.owns = true;

        Ok(())
    }

    pub fn sell(&mut self, wallet: &mut usize, item_flag: ArmorItemFlag, add_to_wallet: bool) -> Result<(), &str> {
        let item_option = self.retrieve_item(item_flag);

        if item_option.is_none() {
            return Err("The item was invalid.");
        }

        let item = item_option.unwrap();

        if !item.owns {
            return Err("You do not own this item.");
        }

        item.owns = false;

        if add_to_wallet {
            *wallet += item.price / 2;
        }

        Ok(())
    }
}
