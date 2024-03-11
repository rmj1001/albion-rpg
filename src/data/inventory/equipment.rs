use serde::{Deserialize, Serialize};

use crate::{panic_screen, prelude::*};

use crate::data::player::Player;

use super::{armor::Armor, weapons::Weapon};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Equipment {
    pub armor: Option<Armor>,
    pub weapon: Option<Weapon>,
}

impl Equipment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn menu(player: &mut Player) {
        page_header("Equipment Manager", Instructions::Keyboard);

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
        page_header("Equipment Manager - Armor", Instructions::Keyboard);

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
        page_header("Equipment Manager - Weapons", Instructions::Keyboard);

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
            player.weapons.wooden_sword.flag.name(),
            player.weapons.bronze_sword.flag.name(),
            player.weapons.iron_sword.flag.name(),
            player.weapons.steel_sword.flag.name(),
            player.weapons.mystic_sword.flag.name(),
            player.weapons.wizard_staff.flag.name(),
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let flag = match choice {
            0 => W::WoodenSword,
            1 => W::BronzeSword,
            2 => W::IronSword,
            3 => W::SteelSword,
            4 => W::MysticSword,
            5 => W::WizardStaff,
            _ => panic_screen!("Out of bounds"),
        };

        let weapon = player.weapons.get(&flag);

        if !weapon.owns {
            println!("You do not own this.");
            pause();
            return;
        }

        weapon.equipped = true;

        println!("Equipped the {}", weapon.flag.name());

        Self::unequip_weapon(player, false);
        player.equipment.weapon = Some(flag);

        pause();
    }

    pub fn unequip_weapon(player: &mut Player, menu_facing: bool) {
        if player.equipment.weapon.is_none() && menu_facing {
            println!("You do not have a weapon equipped.");
            pause();
            return;
        }

        if let Some(equipped_weapon) = &player.equipment.weapon {
            let equipped_weapon = player.weapons.get(equipped_weapon);

            equipped_weapon.equipped = false;

            player.equipment.weapon = None;

            if menu_facing {
                println!("Weapon successfully unequipped.");
                pause();
            }
        }
    }

    pub fn equip_armor(player: &mut Player) {
        use Armor as A;

        let choices = [
            player.armor.leather.flag.name(),
            player.armor.bronze.flag.name(),
            player.armor.iron.flag.name(),
            player.armor.steel.flag.name(),
            player.armor.dragonhide.flag.name(),
            player.armor.mystic.flag.name(),
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let flag = match choice {
            0 => A::Leather,
            1 => A::Bronze,
            2 => A::Iron,
            3 => A::Steel,
            4 => A::Dragonhide,
            5 => A::Mystic,
            _ => panic_screen!("Out of bounds"),
        };

        let armor = player.armor.get(&flag);

        if !armor.owns {
            println!("You do not own this.");
            pause();
            return;
        }

        armor.equipped = true;

        println!("Equipped the {}", armor.flag.name());

        Self::unequip_armor(player, false);
        player.equipment.armor = Some(flag);

        pause();
    }

    pub fn unequip_armor(player: &mut Player, menu_facing: bool) {
        if player.equipment.armor.is_none() && menu_facing {
            println!("You do not have armor equipped.");
            pause();
            return;
        }

        if let Some(equipped_armor) = &player.equipment.armor {
            let equipped_armor = player.armor.get(equipped_armor);

            equipped_armor.equipped = false;

            player.equipment.armor = None;

            if menu_facing {
                println!("Armor successfully unequipped.");
                pause();
            }
        }
    }

    pub fn check_equipment_ownership(player: &mut Player) {
        if let Some(weapon_flag) = &player.equipment.weapon {
            let weapon = player.weapons.get(weapon_flag);

            if !weapon.owns {
                Self::unequip_weapon(player, false);
            }
        }

        if let Some(armor_flag) = &player.equipment.armor {
            let armor = player.armor.get(armor_flag);

            if !armor.owns {
                Self::unequip_armor(player, false);
            }
        }
    }
}
