use crate::{
    data::{inventory::weapons::WeaponsInventory, player::Player},
    prelude::{page_header, select, success, unreachable, Instructions, Printer},
};

pub fn main(player: &mut Player) {
    page_header("Weapons Shop", &Instructions::None);

    WeaponsInventory::shop_table(player);

    let buysell = select(&["1. Purchase", "2. Sell", "NAV: Go Back"], None);

    match buysell {
        0 => purchase(player),
        1 => sell(player),
        2 => crate::menus::game_menu::main(player),
        _ => unreachable(),
    }

    crate::menus::game_menu::main(player);
}

pub fn purchase(player: &mut Player) {
    let flag = WeaponsInventory::select();
    let result = WeaponsInventory::buy(player, &flag, true);

    match result {
        Ok(()) => {
            success(None);
            main(player);
        }
        Err(message) => {
            message.print(true);
            main(player);
        }
    }
}

pub fn sell(player: &mut Player) {
    let flag = WeaponsInventory::select();
    let result = WeaponsInventory::sell(player, &flag, true);

    match result {
        Ok(()) => {
            success(None);
            main(player);
        }
        Err(message) => {
            message.print(true);
            main(player);
        }
    }
}
