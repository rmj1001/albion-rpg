use crate::{
    lib::{
        input::{select_from_str_array, select_from_vector},
        messages::*,
        tui::page_header,
    },
    user::{profile::UserProfile, weapons::WeaponItemFlag},
};

pub fn main(user: &mut UserProfile) {
    page_header("Weapons Shop", crate::lib::tui::HeaderSubtext::None);

    println!("Gold: {}\n", user.bank.wallet);
    user.weapons.print_table();

    let buysell = select_from_str_array(&["1. Purchase", "2. Sell", "NAV: Go Back"], None);

    match buysell {
        0 => purchase(user),
        1 => sell(user),
        2 => crate::menus::game_menu::main(user),
        _ => out_of_bounds::<String>(None),
    }

    crate::menus::game_menu::main(user);
}

pub fn purchase(user: &mut UserProfile) {
    let item = get_item(user);

    let result = user.weapons.purchase(&mut user.bank.wallet, item, true);

    match result {
        Ok(_) => {
            success();
            main(user);
        }
        Err(message) => {
            failure(message);
            main(user);
        }
    }
}

pub fn sell(user: &mut UserProfile) {
    let item = get_item(user);

    let result = user.weapons.sell(&mut user.bank.wallet, item, true);

    match result {
        Ok(_) => {
            success();
            main(user);
        }
        Err(message) => {
            failure(message);
            main(user);
        }
    }
}

fn get_item(user: &mut UserProfile) -> WeaponItemFlag {
    let items: Vec<String> = vec![
        user.weapons.wooden_sword.name.to_string(),
        user.weapons.bronze_sword.name.to_string(),
        user.weapons.iron_sword.name.to_string(),
        user.weapons.steel_sword.name.to_string(),
        user.weapons.mystic_sword.name.to_string(),
        user.weapons.wizard_staff.name.to_string(),
        "NAV: Cancel".to_string(),
    ];

    let length = items.len();

    let select = select_from_vector(items, None);

    if select == length - 1 {
        cancelling();
        main(user);
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
