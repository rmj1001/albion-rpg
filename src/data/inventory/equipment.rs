use serde::{Deserialize, Serialize};

use crate::panic_menu;

use crate::prelude::{page_header, pause, select, unreachable, Instructions};

use crate::data::player::Player;

use super::{armor, weapons};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Equipment {
    pub armor: Option<armor::Types>,
    pub weapon: Option<weapons::Types>,
}

impl Equipment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn menu(player: &mut Player) {
        page_header("Equipment Manager", &Instructions::Keyboard);

        let choice = select(&["1. Weapons", "2. Armor", "NAV: Go Back"], None);

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
            _ => unreachable(),
        }
    }

    pub fn armor_menu(player: &mut Player) {
        page_header("Equipment Manager - Armor", &Instructions::Keyboard);

        player.armor.table();

        let choices: usize = select(&["1. Equip Armor", "2. Un-Equip Armor", "NAV: Go Back"], None);

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
            _ => unreachable(),
        }
    }

    pub fn weapon_menu(player: &mut Player) {
        page_header("Equipment Manager - Weapons", &Instructions::Keyboard);

        player.weapons.table();

        let choices: usize = select(&["1. Equip Weapon", "2. Un-Equip Weapon", "NAV: Go Back"], None);

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
            _ => unreachable(),
        }
    }

    pub fn equip_weapon(player: &mut Player) {
        use weapons::Types as W;

        let choices = [
            player.weapons.wooden_sword.flag.to_string(),
            player.weapons.bronze_sword.flag.to_string(),
            player.weapons.iron_sword.flag.to_string(),
            player.weapons.steel_sword.flag.to_string(),
            player.weapons.mystic_sword.flag.to_string(),
            player.weapons.wizard_staff.flag.to_string(),
        ];

        let choice: usize = select(&choices, None);

        let flag = match choice {
            0 => W::Wooden,
            1 => W::Bronze,
            2 => W::Iron,
            3 => W::Steel,
            4 => W::Mystic,
            5 => W::WizardStaff,
            _ => panic_menu!("Out of bounds"),
        };

        let weapon = player.weapons.get(&flag);

        if !weapon.owns {
            println!("You do not own this.");
            pause();
            return;
        }

        weapon.equipped = true;

        println!("Equipped the {}", weapon.flag);

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
        use armor::Types as A;

        let choices = [
            player.armor.leather.flag.to_string(),
            player.armor.bronze.flag.to_string(),
            player.armor.iron.flag.to_string(),
            player.armor.steel.flag.to_string(),
            player.armor.dragonhide.flag.to_string(),
            player.armor.mystic.flag.to_string(),
        ];

        let choice: usize = select(&choices, None);

        let flag = match choice {
            0 => A::Leather,
            1 => A::Bronze,
            2 => A::Iron,
            3 => A::Steel,
            4 => A::Dragonhide,
            5 => A::Mystic,
            _ => panic_menu!("Out of bounds"),
        };

        let armor = player.armor.get(&flag);

        if !armor.owns {
            println!("You do not own this.");
            pause();
            return;
        }

        armor.equipped = true;

        println!("Equipped the {}", armor.flag);

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
