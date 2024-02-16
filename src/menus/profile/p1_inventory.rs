use crate::{
    data::inventory::equipment::Equipment,
    economy::items,
    utils::{
        input::select_from_str_array,
        messages::*,
        tui::{page_header, press_enter_to_continue, HeaderSubtext},
    },
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Inventory Viewer", HeaderSubtext::None);

    let menu_option = select_from_str_array(
        &[
            "1. Items",
            "2. Weapons",
            "3. Armor",
            "4. Equipment",
            "5. Finances",
            "NAV: Go Back",
        ],
        None,
    );

    match menu_option {
        0 => mundane_inventory(player),
        1 => weapons_inventory(player),
        2 => armor_inventory(player),
        3 => {
            Equipment::menu(player);
            main(player);
        }
        4 => finances(player),
        5 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

pub fn mundane_inventory(player: &mut Player) {
    page_header("Items Inventory", HeaderSubtext::None);

    items::shop::table(player);

    press_enter_to_continue();
    main(player);
}

pub fn weapons_inventory(player: &mut Player) {
    page_header("Weapons Inventory", HeaderSubtext::None);

    player.weapons.table();

    press_enter_to_continue();
    main(player);
}

pub fn armor_inventory(player: &mut Player) {
    page_header("Armor Inventory", HeaderSubtext::None);

    player.armor.table();

    press_enter_to_continue();
    main(player);
}

pub fn finances(player: &mut Player) {
    page_header("Finances", HeaderSubtext::None);

    player.bank.table();

    press_enter_to_continue();
    main(player);
}
