use crate::{
    lib::{
        input::{
            input_generic, invalid_input, out_of_bounds, prompt_input_completion,
            select_from_str_array,
        },
        tui::{page_header, press_enter_to_continue},
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
    let (item, quantity) = get_item_and_quantity(user);

    // TODO: inventory arithmetic here.
}

pub fn sell(user: &mut UserProfile) {
    let (item, quantity) = get_item_and_quantity(user);

    // TODO: inventory arithmetic here.
}

fn get_item_and_quantity(user: &mut UserProfile) -> (String, usize) {
    // TODO: Populate with weapon names
    let items: Vec<String> = vec![];

    let item = prompt_input_completion(
        "Type the name of the item you wish to purchase.",
        items.clone(),
    );

    if !items.contains(&item.to_lowercase()) {
        invalid_input(Some(&item), None, true);
        main(user);
    }

    let quantity: Result<usize, &str> = input_generic::<usize>("Quantity");

    if quantity.is_err() {
        eprintln!("{}", quantity.unwrap());
        press_enter_to_continue();
        main(user);
    }

    (item, quantity.unwrap())
}
