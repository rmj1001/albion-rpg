use crate::{
    data::{
        inventory::{armor, items, weapons},
        player::Player,
    },
    prelude::{page_header, select, success, unreachable, Instructions, Printer},
};

pub fn main(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager", &Instructions::None);

    let manager_option = select(&["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"], None);

    match manager_option {
        0 => items_manager(player),
        1 => weapons_manager(player),
        2 => armor_manager(player),
        3 => super::d1_developer_menu::main(player),
        _ => unreachable(),
    }
}

fn items_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Items", &Instructions::None);

    items::Inventory::shop_table(player);

    let buysell = select(&["1. Add", "2. Subtract", "NAV: Go Back"], None);

    match buysell {
        0 => add_item(player),
        1 => subtract_item(player),
        2 => main(player),
        _ => unreachable(),
    }
}

fn weapons_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Weapons", &Instructions::None);

    weapons::Inventory::shop_table(player);

    let buysell: usize = select(&["1. Own Weapon", "2. Disown Weapon", "NAV: Go Back"], None);

    match buysell {
        0 => own_weapon(player),
        1 => disown_weapon(player),
        2 => main(player),
        _ => unreachable(),
    }
}

fn armor_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Armor", &Instructions::None);

    player.armor.table();

    let buysell = select(&["1. Own Armor", "2. Disown Armor", "NAV: Go Back"], None);

    match buysell {
        0 => own_armor(player),
        1 => disown_armor(player),
        2 => main(player),
        _ => unreachable(),
    }
}

fn add_item(player: &mut Player) {
    if let Ok((flag, quantity)) = items::Inventory::build_transaction() {
        let result = items::Inventory::buy(player, flag, quantity, false);

        match result {
            Ok(()) => {
                success(None);
                items_manager(player);
            }
            Err(message) => {
                message.print(true);
                items_manager(player);
            }
        }
    } else {
        items_manager(player);
    }
}

fn subtract_item(player: &mut Player) {
    if let Ok((flag, quantity)) = items::Inventory::build_transaction() {
        let sell_result = items::Inventory::sell(player, flag, quantity, false);

        match sell_result {
            Ok(()) => {
                success(None);
                items_manager(player);
            }
            Err(message) => {
                message.print(true);
                items_manager(player);
            }
        }
    } else {
        items_manager(player);
    }
}

fn own_weapon(player: &mut Player) {
    let flag = weapons::Inventory::select();
    let result = weapons::Inventory::buy(player, &flag, false);

    match result {
        Ok(()) => {
            success(None);
            weapons_manager(player);
        }
        Err(message) => {
            message.print(true);
            weapons_manager(player);
        }
    }
}

fn disown_weapon(player: &mut Player) {
    let flag = weapons::Inventory::select();
    let result = weapons::Inventory::sell(player, &flag, false);

    match result {
        Ok(()) => {
            success(None);
            weapons_manager(player);
        }
        Err(message) => {
            message.print(true);
            weapons_manager(player);
        }
    }
}

fn own_armor(player: &mut Player) {
    let item = armor::Inventory::select();
    let result = armor::Inventory::buy(player, &item, false);

    match result {
        Ok(()) => {
            success(None);
            armor_manager(player);
        }
        Err(message) => {
            message.print(true);
            armor_manager(player);
        }
    }
}

fn disown_armor(player: &mut Player) {
    let item = armor::Inventory::select();
    let result = armor::Inventory::buy(player, &item, false);

    match result {
        Ok(()) => {
            success(None);
            armor_manager(player);
        }
        Err(message) => {
            message.print(true);
            armor_manager(player);
        }
    }
}
