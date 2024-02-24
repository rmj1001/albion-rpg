use serde::{Deserialize, Serialize};

use crate::utils::{
    input::select_from_str_array,
    messages::out_of_bounds,
    tui::{page_header, press_enter_to_continue, HeaderSubtext},
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
        Self::unequip_weapon(player, false);

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

    pub fn unequip_weapon(player: &mut Player, menu_facing: bool) {
        if player.equipment.weapon.is_none() && menu_facing {
            println!("You do not have a weapon equipped.");
            press_enter_to_continue();
            return;
        } else if player.equipment.weapon.is_none() {
            return;
        }

        let mut equipped_weapon = player.equipment.weapon.clone().unwrap();

        equipped_weapon.equipped = false;

        Self::overwrite_inventory_weapon(equipped_weapon, player);

        player.equipment.weapon = None;

        if menu_facing {
            println!("Weapon successfully unequipped.");
            press_enter_to_continue();
        }
    }

    pub fn overwrite_inventory_weapon(equipped: Weapon, player: &mut Player) {
        let name = equipped.name.clone();
        let weapons = &mut player.weapons;

        use easy_switch::switch;

        switch! {name;
            weapons.wooden_sword.name => weapons.wooden_sword = equipped,
            weapons.bronze_sword.name => weapons.bronze_sword = equipped,
            weapons.iron_sword.name => weapons.iron_sword = equipped,
            weapons.steel_sword.name => weapons.steel_sword = equipped,
            weapons.mystic_sword.name => weapons.mystic_sword = equipped,
            weapons.wizard_staff.name => weapons.wizard_staff = equipped,
        };
    }

    fn equip_armor(player: &mut Player) {
        Self::unequip_armor(player, false);

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

    fn unequip_armor(player: &mut Player, menu_facing: bool) {
        if player.equipment.armor.is_none() && menu_facing {
            println!("You do not have armor equipped.");
            press_enter_to_continue();
            return;
        } else if player.equipment.armor.is_none() {
            return;
        }

        let mut equipped_armor = player.equipment.armor.clone().unwrap();

        equipped_armor.equipped = false;

        Self::overwrite_inventory_armor(equipped_armor, player);

        player.equipment.armor = None;

        if menu_facing {
            println!("Armor successfully unequipped.");
            press_enter_to_continue();
        }
    }

    pub fn overwrite_inventory_armor(equipped: Armor, player: &mut Player) {
        let name = equipped.name.clone();
        let armor = &mut player.armor;

        use easy_switch::switch;

        switch! {name;
            armor.leather.name => armor.leather = equipped,
            armor.bronze.name => armor.bronze = equipped,
            armor.iron.name => armor.iron = equipped,
            armor.dragonhide.name => armor.dragonhide = equipped,
            armor.mystic.name => armor.mystic = equipped,
        };
    }
}
