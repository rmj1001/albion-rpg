use crate::{
    data::{inventory::weapons::WeaponsInventory, player::Player},
    prelude::*,
};

pub fn main(player: &mut Player) {
    page_header("Weapons Shop", crate::utils::tui::Instructions::None);

    WeaponsInventory::shop_table(player);

    let buysell = select_from_str_array(&["1. Purchase", "2. Sell", "NAV: Go Back"], None);

    match buysell {
        0 => purchase(player),
        1 => sell(player),
        2 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }

    crate::menus::game_menu::main(player);
}

pub fn purchase(player: &mut Player) {
    let flag = WeaponsInventory::select();
    let result = WeaponsInventory::buy(player, flag, true);

    match result {
        Ok(_) => {
            success(None);
            main(player);
        }
        Err(message) => {
            message.failure();
            main(player);
        }
    }
}

pub fn sell(player: &mut Player) {
    let flag = WeaponsInventory::select();
    let result = WeaponsInventory::sell(player, flag, true);

    match result {
        Ok(_) => {
            success(None);
            main(player);
        }
        Err(message) => {
            message.failure();
            main(player);
        }
    }
}
