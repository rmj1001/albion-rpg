use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{checkmark, price, table_from_csv};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub equipped: bool,
}

impl Armor {
    pub fn new(name: &str, price: usize, defense: usize, durability: usize) -> Self {
        Self {
            name: format!("{} Armor", name),
            price,
            owns: false,
            equipped: false,
            defense,
            durability,
            default_durability: durability,
        }
    }

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn new() -> ArmorInventory {
        ArmorInventory {
            leather: Armor::new("Leather", 50, 10, 100),
            bronze: Armor::new("Bronze", 200, 30, 200),
            iron: Armor::new("Iron", 500, 50, 300),
            steel: Armor::new("Steel", 750, 100, 500),
            dragonhide: Armor::new("Dragonhide", 1_000, 200, 500),
            mystic: Armor::new("Mystic", 10_000, 1_000, 10_000),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(armor: &Armor) -> String {
            format!(
                "{},{},{},{},{},{},{}",
                armor.name,
                checkmark(armor.owns),
                checkmark(armor.equipped),
                armor.defense,
                armor.durability,
                price(armor.price),
                price(armor.price / 2)
            )
        }

        table_from_csv(vec![
            "Armor,Owned,Equipped,Defense,Durability,Purchase,Sell".to_string(),
            entry(&self.leather),
            entry(&self.bronze),
            entry(&self.iron),
            entry(&self.steel),
            entry(&self.dragonhide),
            entry(&self.mystic),
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
    pub fn toggle_own(&mut self, item_flag: ArmorItemFlag) {
        let item = self.retrieve_item(item_flag);

        if item.is_none() {
            return;
        }

        if let Some(item) = item {
            item.owns = !item.owns;
        }
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
