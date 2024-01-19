use crate::{
    lib::{
        input::{out_of_bounds, select_from_str_array},
        tui::{page_header, HeaderSubtext},
    },
    user::profile::UserProfile,
};

pub fn main(user: &mut UserProfile) {
    page_header("Developer Mode - Inventory Manager", HeaderSubtext::None);

    let manager_option = select_from_str_array(
        &["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"],
        None,
    );

    match manager_option {
        0 => items_manager(user),
        1 => weapons_manager(user),
        2 => armor_manager(user),
        3 => super::d1_developer_menu::main(user),
        _ => out_of_bounds(None),
    }
}

fn items_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Items",
        HeaderSubtext::None,
    );

    // TODO: Items Manager

    main(user);
}

fn weapons_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Weapons",
        HeaderSubtext::None,
    );

    // TODO: Weapons Manager

    main(user);
}

fn armor_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Armor",
        HeaderSubtext::None,
    );

    // TODO: Armor Manager

    main(user);
}
