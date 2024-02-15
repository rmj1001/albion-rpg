use serde::{Deserialize, Serialize};

use crate::utils::{
    input::select_from_str_array,
    messages::out_of_bounds,
    tui::{page_header, press_enter_to_continue, table_from_csv, HeaderSubtext},
};

use crate::data::player::Player;

use super::{armor::Armor, weapons::Weapon};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Equipment {
    pub armor: Option<Armor>,
    pub weapon: Option<Weapon>,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            armor: None,
            weapon: None,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        let mut weapon_string: String = String::new();
        let mut armor_string: String = String::new();

        if self.weapon.is_none() {
            weapon_string = String::from("Weapon: None,Damage: N/A,Durability: N/A");
        } else {
            let weapon = self.weapon.clone().unwrap();

            weapon_string = format!(
                "Weapon: {},Damage: {},Durability: {}",
                weapon.name, weapon.damage, weapon.durability
            );
        }

        if self.armor.is_none() {
            armor_string = String::from("Armor: None,Defense: N/A,Durability: N/A");
        } else {
            let armor = self.armor.clone().unwrap();

            armor_string = format!(
                "Armor: {},Defense: {},Durability: {}",
                armor.name, armor.defense, armor.durability
            )
        }

        table_from_csv(vec![weapon_string, armor_string])
    }

    pub fn menu(player: &mut Player) {
        page_header("Equipment Manager", HeaderSubtext::Keyboard);

        player.equipment.table();

        let choice = select_from_str_array(&["1. Weapons", "2. Armor", "NAV: Go Back"], None);

        match choice {
            0 => {
                Self::weapon_menu(player);
                Self::menu(player);
            }
            1 => {
                Self::armor_menu(player);
                Self::menu(player);
            }
            2 => player.save(), // goes back to whatever menu called it due to recursion
            _ => out_of_bounds(),
        }
    }

    pub fn armor_menu(player: &mut Player) {
        page_header("Equipment Manager - Armor", HeaderSubtext::Keyboard);

        player.armor.table();

        let choices: usize = select_from_str_array(&["1. Equip Armor", "2. Un-Equip Armor", "NAV: Go Back"], None);

        match choices {
            0 => {
                Self::equip_armor(player);
                Self::armor_menu(player);
            }
            1 => {
                Self::unequip_armor(player);
                Self::armor_menu(player);
            }
            2 => {}
            _ => out_of_bounds(),
        }
    }

    pub fn weapon_menu(player: &mut Player) {
        page_header("Equipment Manager - Weapons", HeaderSubtext::Keyboard);

        player.weapons.table();

        let choices: usize = select_from_str_array(&["1. Equip Weapon", "2. Un-Equip Weapon", "NAV: Go Back"], None);

        match choices {
            0 => {
                Self::equip_weapon(player);
                Self::weapon_menu(player);
            }
            1 => {
                Self::unequip_weapon(player);
                Self::weapon_menu(player);
            }
            2 => {}
            _ => out_of_bounds(),
        }
    }

    pub fn equip_weapon(player: &mut Player) {
        let choices = [
            &player.weapons.wooden_sword.name[..],
            &player.weapons.bronze_sword.name[..],
            &player.weapons.iron_sword.name[..],
            &player.weapons.steel_sword.name[..],
            &player.weapons.mystic_sword.name[..],
            &player.weapons.wizard_staff.name[..],
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let weapon_option: Option<&mut Weapon> = match choice {
            0 => Some(&mut player.weapons.wooden_sword),
            1 => Some(&mut player.weapons.bronze_sword),
            2 => Some(&mut player.weapons.iron_sword),
            3 => Some(&mut player.weapons.steel_sword),
            4 => Some(&mut player.weapons.mystic_sword),
            5 => Some(&mut player.weapons.wizard_staff),
            _ => None,
        };

        if weapon_option.is_none() {
            out_of_bounds();
        }

        let weapon = weapon_option.unwrap();

        if !weapon.owns {
            println!("You do not own this.");
            press_enter_to_continue();
        } else {
            weapon.equipped = true;
            player.equipment.weapon = Some(weapon.clone());

            println!("Equipped the {}", weapon.name);
            press_enter_to_continue();
        }
    }

    pub fn unequip_weapon(player: &mut Player) {
        if player.equipment.weapon.is_none() {
            println!("You do not have a weapon equipped.");
            press_enter_to_continue();
            return;
        }

        let mut equipped_weapon = player.equipment.weapon.clone().unwrap();

        equipped_weapon.equipped = false;

        Self::overwrite_inventory_weapon(player.equipment.weapon.clone().unwrap(), player);

        player.equipment.weapon = None;
        println!("Weapon successfully unequipped.");
        press_enter_to_continue();
    }

    pub fn overwrite_inventory_weapon(equipped: Weapon, player: &mut Player) {
        let name = equipped.name.clone();
        let weapons = &mut player.weapons;

        if name == weapons.wooden_sword.name {
            weapons.wooden_sword = equipped;
        } else if name == weapons.bronze_sword.name {
            weapons.bronze_sword = equipped;
        } else if name == weapons.iron_sword.name {
            weapons.iron_sword = equipped;
        } else if name == weapons.steel_sword.name {
            weapons.steel_sword = equipped;
        } else if name == weapons.mystic_sword.name {
            weapons.mystic_sword = equipped;
        } else if name == weapons.wizard_staff.name {
            weapons.wizard_staff = equipped;
        }
    }

    fn equip_armor(player: &mut Player) {
        let choices = [
            &player.armor.leather.name[..],
            &player.armor.bronze.name[..],
            &player.armor.iron.name[..],
            &player.armor.dragonhide.name[..],
            &player.armor.mystic.name[..],
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let option: Option<&mut Armor> = match choice {
            0 => Some(&mut player.armor.leather),
            1 => Some(&mut player.armor.bronze),
            2 => Some(&mut player.armor.iron),
            3 => Some(&mut player.armor.dragonhide),
            4 => Some(&mut player.armor.mystic),
            _ => None,
        };

        if option.is_none() {
            out_of_bounds();
        }

        let armor: &mut Armor = option.unwrap();

        if !armor.owns {
            println!("You do not own this.");
            press_enter_to_continue();
        } else {
            armor.equipped = true;
            player.equipment.armor = Some(armor.clone());

            println!("Equipped {}", armor.name);
            press_enter_to_continue();
        }
    }

    fn unequip_armor(player: &mut Player) {
        if player.equipment.armor.is_none() {
            println!("You do not have armor equipped.");
            press_enter_to_continue();
            return;
        }

        let mut equipped_armor = player.equipment.armor.clone().unwrap();

        equipped_armor.equipped = false;

        Self::overwrite_inventory_armor(equipped_armor, player);

        player.equipment.armor = None;
        println!("Armor successfully unequipped.");
        press_enter_to_continue();
    }

    pub fn overwrite_inventory_armor(equipped: Armor, player: &mut Player) {
        let name = equipped.name.clone();
        let armor = &mut player.armor;

        if name == armor.leather.name {
            armor.leather = equipped;
        } else if name == armor.bronze.name {
            armor.bronze = equipped
        } else if name == armor.iron.name {
            armor.iron = equipped;
        } else if name == armor.dragonhide.name {
            armor.dragonhide = equipped;
        } else if name == armor.mystic.name {
            armor.mystic = equipped;
        }
    }
}
