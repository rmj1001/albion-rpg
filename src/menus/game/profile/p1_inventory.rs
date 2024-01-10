use crate::lib::{
    input::{out_of_bounds, selector},
    tui::{page_header, press_enter_to_continue, HeaderInstructions},
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header("Inventory Viewer", HeaderInstructions::None);

    let menu_option = selector(
        &["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"],
        0,
        None,
    );

    match menu_option {
        0 => mundane_inventory(user),
        1 => weapons_inventory(user),
        2 => armor_inventory(user),
        3 => crate::menus::game::main::menu(user),
        _ => out_of_bounds(None),
    }
}

pub fn mundane_inventory(user: &mut UserProfile) {
    page_header("Items Inventory", HeaderInstructions::None);

    // TODO: Mundane Items Inventory

    press_enter_to_continue();
    main(user);
}

pub fn weapons_inventory(user: &mut UserProfile) {
    page_header("Weapons Inventory", HeaderInstructions::None);

    // TODO: Weapons Inventory

    press_enter_to_continue();
    main(user);
}

pub fn armor_inventory(user: &mut UserProfile) {
    page_header("Armor Inventory", HeaderInstructions::None);

    // TODO: Armor Inventory

    press_enter_to_continue();
    main(user);
}
