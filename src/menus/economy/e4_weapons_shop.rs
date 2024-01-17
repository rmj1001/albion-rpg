use crate::{
    lib::{
        input::{self, out_of_bounds, select_from_str_array},
        tui::page_header,
    },
    user::profile::UserProfile,
};

pub fn main(user: &mut UserProfile) {
    page_header("Weapons Shop", crate::lib::tui::HeaderSubtext::None);

    user.weapons.print_table();

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
    let (item, quantity) = get_item_and_quantity(user)
        .expect("get_item_and_quantity() didn't go back to the main menu.");

    // TODO: inventory arithmetic here.
}

pub fn sell(user: &mut UserProfile) {
    let (item, quantity) = get_item_and_quantity(user)
        .expect("get_item_and_quantity() didn't go back to the main menu.");

    // TODO: inventory arithmetic here.
}

fn get_item_and_quantity(user: &mut UserProfile) -> Option<(String, usize)> {
    let items: Vec<String> = vec![
        "wooden sword".to_string(),
        "bronze sword".to_string(),
        "iron sword".to_string(),
        "steel sword".to_string(),
        "mystic sword".to_string(),
        "wizard staff".to_string(),
    ];

    let result = input::get_item_and_quantity(items);

    match result {
        Ok(tuple) => Some(tuple),
        Err(_) => {
            main(user);
            None
        }
    }
}
