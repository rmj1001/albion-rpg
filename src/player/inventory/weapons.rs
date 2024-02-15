use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::utils::tui::{pretty_bool, print_table};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Weapon {
    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);

        if self.durability < random_damage {
            self.break_weapon();
        } else {
            self.durability -= random_damage;
        }
    }

    pub fn break_weapon(&mut self) {
        println!("Your {} broke!", self.name);
        self.owns = false;
        self.durability = self.default_durability;
    }
}

pub enum WeaponItemFlag {
    WoodenSword,
    BronzeSword,
    IronSword,
    SteelSword,
    MysticSword,
    WizardStaff,
    InvalidItem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeaponsInventory {
    pub wooden_sword: Weapon,
    pub bronze_sword: Weapon,
    pub iron_sword: Weapon,
    pub steel_sword: Weapon,
    pub mystic_sword: Weapon,
    pub wizard_staff: Weapon,
}

impl WeaponsInventory {
    pub fn print_table(&self) {
        print_table(vec![
            "Weapon,Purchased,Buy Price,Sale Price".to_string(),
            format!(
                "{},{},{},{}",
                self.wooden_sword.name,
                pretty_bool(self.wooden_sword.owns),
                self.wooden_sword.price,
                self.wooden_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.bronze_sword.name,
                pretty_bool(self.bronze_sword.owns),
                self.bronze_sword.price,
                self.bronze_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.iron_sword.name,
                pretty_bool(self.iron_sword.owns),
                self.iron_sword.price,
                self.iron_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.steel_sword.name,
                pretty_bool(self.steel_sword.owns),
                self.steel_sword.price,
                self.steel_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.mystic_sword.name,
                pretty_bool(self.mystic_sword.owns),
                self.mystic_sword.price,
                self.mystic_sword.price / 2
            ),
            format!(
                "{},{},{},{}",
                self.wizard_staff.name,
                pretty_bool(self.wizard_staff.owns),
                self.wizard_staff.price,
                self.wizard_staff.price / 2
            ),
        ])
    }

    pub fn retrieve_item(&mut self, item_flag: WeaponItemFlag) -> Option<&mut Weapon> {
        match item_flag {
            WeaponItemFlag::BronzeSword => Some(&mut self.bronze_sword),
            WeaponItemFlag::IronSword => Some(&mut self.iron_sword),
            WeaponItemFlag::MysticSword => Some(&mut self.mystic_sword),
            WeaponItemFlag::SteelSword => Some(&mut self.steel_sword),
            WeaponItemFlag::WizardStaff => Some(&mut self.wizard_staff),
            WeaponItemFlag::WoodenSword => Some(&mut self.wooden_sword),
            WeaponItemFlag::InvalidItem => None,
        }
    }

    /// For use in developer mode only
    pub fn own_item(&mut self, item_flag: WeaponItemFlag, flag: bool) {
        let item_result = self.retrieve_item(item_flag);

        if item_result.is_none() {
            return;
        }

        item_result.unwrap().owns = flag;
    }

    pub fn purchase(
        &mut self,
        wallet: &mut usize,
        item_flag: WeaponItemFlag,
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

    pub fn sell(&mut self, wallet: &mut usize, item_flag: WeaponItemFlag, add_to_wallet: bool) -> Result<(), &str> {
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
