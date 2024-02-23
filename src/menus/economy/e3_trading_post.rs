use crate::{
    data::player::Player,
    economy::items::shop,
    utils::{input::select_from_str_array, messages::*, tui::page_header},
};

pub fn main(player: &mut Player) {
    page_header("Trading Post", crate::utils::tui::HeaderSubtext::None);

    shop::table(player);
    println!("Gold: {}\n", player.bank.wallet);

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
    let (item_flag, quantity) = shop::build_transaction();

    let result = shop::purchase(player, item_flag, quantity, true);

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
    let (item_flag, quantity) = shop::build_transaction();

    let result = shop::sell(player, item_flag, quantity, true);

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
