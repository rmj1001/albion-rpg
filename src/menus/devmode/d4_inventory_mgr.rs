use crate::{
    data::{inventory::weapons::WeaponItemFlag, player::Player},
    economy::{
        armor::{self, shop::build_transaction},
        items,
    },
    utils::{
        input::{select_from_str_array, select_from_vector},
        messages::*,
        tui::{page_header, HeaderSubtext},
    },
};

pub fn main(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager", HeaderSubtext::None);

    let manager_option = select_from_str_array(&["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"], None);

    match manager_option {
        0 => items_manager(player),
        1 => weapons_manager(player),
        2 => armor_manager(player),
        3 => super::d1_developer_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn items_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Items", HeaderSubtext::None);

    items::shop::table(player);

    let buysell = select_from_str_array(&["1. Add", "2. Subtract", "NAV: Go Back"], None);

    match buysell {
        0 => add_item(player),
        1 => subtract_item(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn add_item(player: &mut Player) {
        let (flag, quantity) = items::shop::build_transaction();
        let result = items::shop::purchase(player, flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                items_manager(player);
            }
            Err(message) => {
                failure(message);
                items_manager(player);
            }
        }
    }

    pub fn subtract_item(player: &mut Player) {
        let (flag, quantity) = items::shop::build_transaction();

        let result = items::shop::sell(player, flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                items_manager(player);
            }
            Err(message) => {
                failure(message);
                items_manager(player);
            }
        }
    }
}

fn weapons_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Weapons", HeaderSubtext::None);

    player.weapons.table();

    let buysell: usize = select_from_str_array(&["1. Own Weapon", "2. Disown Weapon", "NAV: Go Back"], None);

    match buysell {
        0 => own_weapon(player),
        1 => disown_weapon(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn own_weapon(player: &mut Player) {
        let item = get_item(player);

        let result: Result<(), String> = player.weapons.purchase(&mut player.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(player);
            }
            Err(message) => {
                failure(message);
                weapons_manager(player);
            }
        }
    }

    pub fn disown_weapon(player: &mut Player) {
        let item = get_item(player);

        let result = player.weapons.sell(&mut player.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(player);
            }
            Err(message) => {
                failure(message);
                weapons_manager(player);
            }
        }
    }

    fn get_item(player: &mut Player) -> WeaponItemFlag {
        let items: Vec<String> = vec![
            player.weapons.wooden_sword.name.to_string(),
            player.weapons.bronze_sword.name.to_string(),
            player.weapons.iron_sword.name.to_string(),
            player.weapons.steel_sword.name.to_string(),
            player.weapons.mystic_sword.name.to_string(),
            player.weapons.wizard_staff.name.to_string(),
            "NAV: Cancel".to_string(),
        ];

        let length = items.len();

        let select = select_from_vector(items, None);

        if select == length - 1 {
            cancelling();
            weapons_manager(player);
            return WeaponItemFlag::InvalidItem;
        }

        match select {
            0 => WeaponItemFlag::WoodenSword,
            1 => WeaponItemFlag::BronzeSword,
            2 => WeaponItemFlag::IronSword,
            3 => WeaponItemFlag::SteelSword,
            4 => WeaponItemFlag::MysticSword,
            5 => WeaponItemFlag::WizardStaff,
            _ => WeaponItemFlag::InvalidItem,
        }
    }
}

fn armor_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Armor", HeaderSubtext::None);

    armor::shop::table(player);

    let buysell = select_from_str_array(&["1. Own Armor", "2. Disown Armor", "NAV: Go Back"], None);

    match buysell {
        0 => own_armor(player),
        1 => disown_armor(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn own_armor(player: &mut Player) {
        let item = build_transaction();
        let result = armor::shop::purchase(player, item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(player);
            }
            Err(message) => {
                failure(message);
                armor_manager(player);
            }
        }
    }

    pub fn disown_armor(player: &mut Player) {
        let item = build_transaction();
        let result = armor::shop::purchase(player, item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(player);
            }
            Err(message) => {
                failure(message);
                armor_manager(player);
            }
        }
    }
}
