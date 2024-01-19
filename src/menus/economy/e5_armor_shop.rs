use crate::{
    lib::{
        input::{select_from_str_array, select_from_vector},
        stdmsgs::*,
        tui::page_header,
    },
    user::{armor::ArmorItemFlag, profile::UserProfile},
};

pub fn main(user: &mut UserProfile) {
    page_header("Armor Shop", crate::lib::tui::HeaderSubtext::None);

    println!("Gold: {}\n", user.bank.wallet);
    user.armor.print_table();

    let buysell = select_from_str_array(&["1. Purchase", "2. Sell", "NAV: Go Back"], None);

    match buysell {
        0 => purchase(user),
        1 => sell(user),
        2 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(None),
    }

    crate::menus::game_menu::main(user);
}

pub fn purchase(user: &mut UserProfile) {
    let item = get_item(user);

    let result = user.armor.purchase(&mut user.bank.wallet, item, true);

    match result {
        Ok(_) => {
            success();
            main(user);
        }
        Err(message) => {
            error(message);
            main(user);
        }
    }
}

pub fn sell(user: &mut UserProfile) {
    let item = get_item(user);

    let result = user.armor.purchase(&mut user.bank.wallet, item, true);

    match result {
        Ok(_) => {
            success();
            main(user);
        }
        Err(message) => {
            error(message);
            main(user);
        }
    }
}

fn get_item(user: &mut UserProfile) -> ArmorItemFlag {
    let items: Vec<String> = vec![
        user.armor.leather.name.to_string(),
        user.armor.bronze.name.to_string(),
        user.armor.iron.name.to_string(),
        user.armor.steel.name.to_string(),
        user.armor.dragonhide.name.to_string(),
        user.armor.mystic.name.to_string(),
        "NAV: Cancel".to_string(),
    ];

    let length = items.len();

    let select = select_from_vector(items, None);

    if select == length - 1 {
        cancelling();
        main(user);
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
