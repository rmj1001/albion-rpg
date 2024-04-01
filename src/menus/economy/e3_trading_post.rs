use crate::{
    data::{inventory::items, player::Player},
    prelude::{error::Printer, page_header, select, success, unreachable, Instructions},
};

pub fn main(player: &mut Player) {
    page_header("Trading Post", &Instructions::None);

    items::Inventory::shop_table(player);
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
    match items::Inventory::build_transaction() {
        Ok((item_flag, quantity)) => {
            let bought = items::Inventory::buy(player, item_flag, quantity, true);

            match bought {
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
        Err(message) => {
            message.print(true);
            main(player);
        }
    }
}

pub fn sell(player: &mut Player) {
    match items::Inventory::build_transaction() {
        Ok((item_flag, quantity)) => {
            let sold = items::Inventory::sell(player, item_flag, quantity, true);

            match sold {
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
        Err(message) => {
            message.print(true);
            main(player);
        }
    }
}
