use crate::{
    data::{
        inventory::{armor::ArmorInventory, items::ItemInventory, weapons::WeaponsInventory},
        player::Player,
    },
    utils::{
        input::select_from_str_array,
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

    ItemInventory::print_shop(player);

    let buysell = select_from_str_array(&["1. Add", "2. Subtract", "NAV: Go Back"], None);

    match buysell {
        0 => add_item(player),
        1 => subtract_item(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn add_item(player: &mut Player) {
        if let Ok((flag, quantity)) = ItemInventory::build_transaction() {
            let result = ItemInventory::buy(player, flag, quantity, false);

            match result {
                Ok(_) => {
                    success();
                    items_manager(player);
                }
                Err(message) => {
                    message.failure();
                    items_manager(player);
                }
            }
        } else {
            items_manager(player);
        }
    }

    pub fn subtract_item(player: &mut Player) {
        if let Ok((flag, quantity)) = ItemInventory::build_transaction() {
            let sell_result = ItemInventory::sell(player, flag, quantity, false);

            match sell_result {
                Ok(_) => {
                    success();
                    items_manager(player);
                }
                Err(message) => {
                    message.failure();
                    items_manager(player);
                }
            }
        } else {
            items_manager(player);
        }
    }
}

fn weapons_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Weapons", HeaderSubtext::None);

    WeaponsInventory::print_shop(player);

    let buysell: usize = select_from_str_array(&["1. Own Weapon", "2. Disown Weapon", "NAV: Go Back"], None);

    match buysell {
        0 => own_weapon(player),
        1 => disown_weapon(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn own_weapon(player: &mut Player) {
        let flag = WeaponsInventory::picker();
        let result = WeaponsInventory::buy(player, flag, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(player);
            }
            Err(message) => {
                message.failure();
                weapons_manager(player);
            }
        }
    }

    pub fn disown_weapon(player: &mut Player) {
        let flag = WeaponsInventory::picker();
        let result = WeaponsInventory::sell(player, flag, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(player);
            }
            Err(message) => {
                message.failure();
                weapons_manager(player);
            }
        }
    }
}

fn armor_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Armor", HeaderSubtext::None);

    player.armor.table();

    let buysell = select_from_str_array(&["1. Own Armor", "2. Disown Armor", "NAV: Go Back"], None);

    match buysell {
        0 => own_armor(player),
        1 => disown_armor(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn own_armor(player: &mut Player) {
        let item = ArmorInventory::picker();
        let result = ArmorInventory::buy(player, &item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(player);
            }
            Err(message) => {
                message.failure();
                armor_manager(player);
            }
        }
    }

    pub fn disown_armor(player: &mut Player) {
        let item = ArmorInventory::picker();
        let result = ArmorInventory::buy(player, &item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(player);
            }
            Err(message) => {
                message.failure();
                armor_manager(player);
            }
        }
    }
}
