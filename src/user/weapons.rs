use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::lib::tui::{pretty_bool, print_table};

use super::profile::UserProfile;

#[derive(Serialize, Deserialize, Debug)]
pub struct Weapon {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub equipped: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Weapon {
    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability == 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

pub enum WeaponItemFlag {
    WoodenSword,
    BronzeSword,
    IronSword,
    SteelSword,
    MysticSword,
    WizardStaff,
}

#[derive(Serialize, Deserialize, Debug)]
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
            "Weapon,Purchased,Equipped,Buy Price,Sale Price".to_string(),
            format!(
                "{},{},{},{},{}",
                self.wooden_sword.name,
                pretty_bool(self.wooden_sword.owns),
                pretty_bool(self.wooden_sword.equipped),
                self.wooden_sword.price,
                self.wooden_sword.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.bronze_sword.name,
                pretty_bool(self.bronze_sword.owns),
                pretty_bool(self.bronze_sword.equipped),
                self.bronze_sword.price,
                self.bronze_sword.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.iron_sword.name,
                pretty_bool(self.iron_sword.owns),
                pretty_bool(self.iron_sword.equipped),
                self.iron_sword.price,
                self.iron_sword.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.steel_sword.name,
                pretty_bool(self.steel_sword.owns),
                pretty_bool(self.steel_sword.equipped),
                self.steel_sword.price,
                self.steel_sword.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.mystic_sword.name,
                pretty_bool(self.mystic_sword.owns),
                pretty_bool(self.mystic_sword.equipped),
                self.mystic_sword.price,
                self.mystic_sword.price / 2
            ),
            format!(
                "{},{},{},{},{}",
                self.wizard_staff.name,
                pretty_bool(self.wizard_staff.owns),
                pretty_bool(self.wizard_staff.equipped),
                self.wizard_staff.price,
                self.wizard_staff.price / 2
            ),
        ])
    }

    pub fn retrieve_item(&mut self, item_flag: WeaponItemFlag) -> &mut Weapon {
        match item_flag {
            WeaponItemFlag::BronzeSword => &mut self.bronze_sword,
            WeaponItemFlag::IronSword => &mut self.iron_sword,
            WeaponItemFlag::MysticSword => &mut self.mystic_sword,
            WeaponItemFlag::SteelSword => &mut self.steel_sword,
            WeaponItemFlag::WizardStaff => &mut self.wizard_staff,
            WeaponItemFlag::WoodenSword => &mut self.wooden_sword,
        }
    }

    /// For use in developer mode only
    pub fn own_item(&mut self, item_flag: WeaponItemFlag, flag: bool) {
        let item = self.retrieve_item(item_flag);

        item.owns = flag;
    }

    pub fn purchase(
        &mut self,
        user: &mut UserProfile,
        item_flag: WeaponItemFlag,
    ) -> Result<(), String> {
        let item = self.retrieve_item(item_flag);

        if item.price > user.bank.wallet {
            return Err("You do not have enough gold to purchase this.".to_string());
        }

        item.owns = true;
        user.bank.wallet -= item.price;
        Ok(())
    }

    pub fn sell(
        &mut self,
        user: &mut UserProfile,
        item_flag: WeaponItemFlag,
    ) -> Result<(), String> {
        let item = self.retrieve_item(item_flag);

        if !item.owns {
            return Err("You do not own this item.".to_string());
        }

        item.owns = false;
        user.bank.wallet += item.price / 2;
        Ok(())
    }
}
