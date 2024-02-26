use serde::{Deserialize, Serialize};

use crate::utils::{
    input::select_from_str_array,
    messages::out_of_bounds,
    tui::{page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::data::player::Player;

use super::{armor::Armor, weapons::Weapon};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Equipment {
    pub armor: Option<Armor>,
    pub weapon: Option<Weapon>,
}

impl Equipment {
    pub fn new() -> Equipment {
        Equipment {
            armor: None,
            weapon: None,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn menu(player: &mut Player) {
        page_header("Equipment Manager", HeaderSubtext::Keyboard);

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
                Self::unequip_armor(player, true);
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
                Self::unequip_weapon(player, true);
                Self::weapon_menu(player);
            }
            2 => {}
            _ => out_of_bounds(),
        }
    }

    pub fn equip_weapon(player: &mut Player) {
        use Weapon as W;

        let choices = [
            &player.weapons.wooden_sword.name[..],
            &player.weapons.bronze_sword.name[..],
            &player.weapons.iron_sword.name[..],
            &player.weapons.steel_sword.name[..],
            &player.weapons.mystic_sword.name[..],
            &player.weapons.wizard_staff.name[..],
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let flag = match choice {
            0 => W::WoodenSword,
            1 => W::BronzeSword,
            2 => W::IronSword,
            3 => W::SteelSword,
            4 => W::MysticSword,
            5 => W::WizardStaff,
            _ => panic!("Out of bounds"),
        };

        let weapon = player.weapons.get(&flag);

        if !weapon.owns {
            println!("You do not own this.");
            press_enter_to_continue();
            return;
        }

        weapon.equipped = true;

        println!("Equipped the {}", weapon.name);

        Self::unequip_weapon(player, false);
        player.equipment.weapon = Some(flag);

        press_enter_to_continue();
    }

    pub fn unequip_weapon(player: &mut Player, menu_facing: bool) {
        if player.equipment.weapon.is_none() && menu_facing {
            println!("You do not have a weapon equipped.");
            press_enter_to_continue();
            return;
        }

        if player.equipment.weapon.is_none() {
            return;
        }

        let flag = player.equipment.weapon.as_ref().unwrap().clone();
        let equipped_weapon = player.weapons.get(&flag);

        equipped_weapon.equipped = false;

        player.equipment.weapon = None;

        if menu_facing {
            println!("Weapon successfully unequipped.");
            press_enter_to_continue();
        }
    }

    pub fn equip_armor(player: &mut Player) {
        use Armor as A;

        let choices = [
            &player.armor.leather.name[..],
            &player.armor.bronze.name[..],
            &player.armor.iron.name[..],
            &player.armor.steel.name[..],
            &player.armor.dragonhide.name[..],
            &player.armor.mystic.name[..],
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let flag = match choice {
            0 => A::Leather,
            1 => A::Bronze,
            2 => A::Iron,
            3 => A::Steel,
            4 => A::Dragonhide,
            5 => A::Mystic,
            _ => panic!("Out of bounds"),
        };

        let armor = player.armor.get(&flag);

        if !armor.owns {
            println!("You do not own this.");
            press_enter_to_continue();
            return;
        }

        armor.equipped = true;

        println!("Equipped the {}", armor.name);
        player.equipment.armor = Some(flag);
        Self::unequip_armor(player, false);

        press_enter_to_continue();
    }

    pub fn unequip_armor(player: &mut Player, menu_facing: bool) {
        if player.equipment.armor.is_none() && menu_facing {
            println!("You do not have armor equipped.");
            press_enter_to_continue();
            return;
        }

        if player.equipment.armor.is_none() {
            return;
        }

        let flag = player.equipment.armor.as_ref().unwrap().clone();
        let equipped_armor = player.armor.get(&flag);

        equipped_armor.equipped = false;

        player.equipment.armor = None;

        if menu_facing {
            println!("Armor successfully unequipped.");
            press_enter_to_continue();
        }
    }

    pub fn check_equipment_ownership(player: &mut Player) {
        if let Some(weapon_flag) = player.equipment.weapon.clone() {
            let weapon = player.weapons.get(&weapon_flag);

            if !weapon.owns {
                Self::unequip_weapon(player, false);
            }
        }

        if let Some(armor_flag) = player.equipment.armor.clone() {
            let armor = player.armor.get(&armor_flag);

            if !armor.owns {
                Self::unequip_armor(player, false);
            }
        }
    }
}
