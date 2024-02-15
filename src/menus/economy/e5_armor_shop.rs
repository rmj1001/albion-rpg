use crate::{
    data::inventory::armor::ArmorItemFlag,
    utils::{
        input::{select_from_str_array, select_from_vector},
        messages::*,
        tui::page_header,
    },
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Armor Shop", crate::utils::tui::HeaderSubtext::None);

    println!("Gold: {}\n", player.bank.wallet);
    player.armor.table();

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
    let item = get_item(player);

    let result = player.armor.purchase(&mut player.bank.wallet, item, true);

    match result {
        Ok(_) => {
            success();
            main(player);
        }
        Err(message) => {
            failure(message);
            main(player);
        }
    }
}

pub fn sell(player: &mut Player) {
    let item = get_item(player);

    let result = player.armor.sell(&mut player.bank.wallet, item, true);

    match result {
        Ok(_) => {
            success();
            main(player);
        }
        Err(message) => {
            failure(message);
            main(player);
        }
    }
}

fn get_item(player: &mut Player) -> ArmorItemFlag {
    let items: Vec<String> = vec![
        player.armor.leather.name.to_string(),
        player.armor.bronze.name.to_string(),
        player.armor.iron.name.to_string(),
        player.armor.steel.name.to_string(),
        player.armor.dragonhide.name.to_string(),
        player.armor.mystic.name.to_string(),
        "NAV: Cancel".to_string(),
    ];

    let length = items.len();

    let select = select_from_vector(items, None);

    if select == length - 1 {
        cancelling();
        main(player);
        return ArmorItemFlag::InvalidItem;
    }

    match select {
        0 => ArmorItemFlag::Leather,
        1 => ArmorItemFlag::Bronze,
        2 => ArmorItemFlag::Iron,
        3 => ArmorItemFlag::Steel,
        4 => ArmorItemFlag::DragonHide,
        5 => ArmorItemFlag::Mystic,
        _ => ArmorItemFlag::InvalidItem,
    }
}
