use crate::{
    data::player::Player,
    economy::weapons,
    utils::{input::select_from_str_array, messages::*, tui::page_header},
};

pub fn main(player: &mut Player) {
    page_header("Weapons Shop", crate::utils::tui::HeaderSubtext::None);

    weapons::shop::table(player);

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
    let flag = weapons::shop::picker();
    let result = weapons::shop::buy(player, flag, true);

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
    let flag = weapons::shop::picker();
    let result = weapons::shop::sell(player, flag, true);

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
