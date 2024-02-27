use crate::data::inventory::armor::ArmorInventory;
use crate::utils::{input::select_from_str_array, messages::*, tui::page_header};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Armor Shop", crate::utils::tui::HeaderSubtext::None);

    ArmorInventory::shop_table(player);

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
    let flag = ArmorInventory::select();
    let result = ArmorInventory::buy(player, &flag, true);

    match result {
        Ok(_) => {
            success();
            main(player);
        }
        Err(message) => {
            message.failure();
            main(player);
        }
    }
}

pub fn sell(player: &mut Player) {
    let flag = ArmorInventory::select();
    let result = ArmorInventory::sell(player, &flag, true);

    match result {
        Ok(_) => {
            success();
            main(player);
        }
        Err(message) => {
            message.failure();
            main(player);
        }
    }
}
