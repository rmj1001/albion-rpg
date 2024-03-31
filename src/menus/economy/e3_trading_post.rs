use crate::{
    data::{inventory::items::ItemInventory, player::Player},
    prelude::*,
};

pub fn main(player: &mut Player) {
    page_header("Trading Post", crate::utils::tui::Instructions::None);

    ItemInventory::shop_table(player);
    println!("Gold: {}\n", player.bank.wallet);

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
    match ItemInventory::build_transaction() {
        Ok((item_flag, quantity)) => {
            let bought = ItemInventory::buy(player, item_flag, quantity, true);

            match bought {
                Ok(_) => {
                    success(None);
                    main(player);
                }
                Err(message) => {
                    message.print(true);
                    main(player);
                }
            }
        }
        Err(message) => {
            message.print(true);
            main(player);
        }
    }
}

pub fn sell(player: &mut Player) {
    match ItemInventory::build_transaction() {
        Ok((item_flag, quantity)) => {
            let sold = ItemInventory::sell(player, item_flag, quantity, true);

            match sold {
                Ok(_) => {
                    success(None);
                    main(player);
                }
                Err(message) => {
                    message.print(true);
                    main(player);
                }
            }
        }
        Err(message) => {
            message.print(true);
            main(player);
        }
    }
}
