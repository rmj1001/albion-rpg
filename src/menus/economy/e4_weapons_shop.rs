use crate::{
    player::{inventory::weapons::WeaponItemFlag, profile::Player},
    utils::{
        input::{select_from_str_array, select_from_vector},
        messages::*,
        tui::page_header,
    },
};

pub fn main(player: &mut Player) {
    page_header("Weapons Shop", crate::utils::tui::HeaderSubtext::None);

    println!("Gold: {}\n", player.bank.wallet);
    player.weapons.print_table();

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

    let result = player.weapons.purchase(&mut player.bank.wallet, item, true);

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

    let result = player.weapons.sell(&mut player.bank.wallet, item, true);

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

fn get_item(player: &mut Player) -> WeaponItemFlag {
    let items: Vec<String> = vec![
        player.weapons.wooden_sword.name.to_string(),
        player.weapons.bronze_sword.name.to_string(),
        player.weapons.iron_sword.name.to_string(),
        player.weapons.steel_sword.name.to_string(),
        player.weapons.mystic_sword.name.to_string(),
        player.weapons.wizard_staff.name.to_string(),
        "NAV: Cancel".to_string(),
    ];

    let length = items.len();

    let select = select_from_vector(items, None);

    if select == length - 1 {
        cancelling();
        main(player);
        return WeaponItemFlag::InvalidItem;
    }

    match select {
        0 => WeaponItemFlag::WoodenSword,
        1 => WeaponItemFlag::BronzeSword,
        2 => WeaponItemFlag::IronSword,
        3 => WeaponItemFlag::SteelSword,
        4 => WeaponItemFlag::MysticSword,
        5 => WeaponItemFlag::WizardStaff,
        _ => WeaponItemFlag::InvalidItem,
    }
}
