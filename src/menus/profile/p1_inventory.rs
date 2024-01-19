use crate::lib::{
    input::select_from_str_array,
    messages::*,
    tui::{page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header("Inventory Viewer", HeaderSubtext::None);

    let menu_option = select_from_str_array(
        &[
            "1. Items",
            "2. Weapons",
            "3. Armor",
            "4. Finances",
            "NAV: Go Back",
        ],
        None,
    );

    match menu_option {
        0 => mundane_inventory(user),
        1 => weapons_inventory(user),
        2 => armor_inventory(user),
        3 => finances(user),
        4 => crate::menus::game_menu::main(user),
        _ => out_of_bounds::<String>(None),
    }
}

pub fn mundane_inventory(user: &mut UserProfile) {
    page_header("Items Inventory", HeaderSubtext::None);

    user.inventory.print_table();

    press_enter_to_continue();
    main(user);
}

pub fn weapons_inventory(user: &mut UserProfile) {
    page_header("Weapons Inventory", HeaderSubtext::None);

    user.weapons.print_table();

    // TODO: Equip/Unequip weapons

    press_enter_to_continue();
    main(user);
}

pub fn armor_inventory(user: &mut UserProfile) {
    page_header("Armor Inventory", HeaderSubtext::None);

    user.armor.print_table();

    // TODO: Equip/Unequip armor

    press_enter_to_continue();
    main(user);
}

pub fn finances(user: &mut UserProfile) {
    page_header("Finances", HeaderSubtext::None);

    user.bank.print_table();

    press_enter_to_continue();
    main(user);
}
