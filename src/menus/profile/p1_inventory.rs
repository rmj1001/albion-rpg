use crate::{
    data::inventory::{equipment::Equipment, items::ItemInventory},
    utils::{
        input::select_from_str_array,
        messages::*,
        tui::{page_header, press_enter_to_continue, HeaderSubtext},
    },
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Inventory Viewer", HeaderSubtext::None);

    let menu_option = select_from_str_array(&["1. Items", "2. Equipment", "3. Finances", "NAV: Go Back"], None);

    match menu_option {
        0 => mundane_inventory(player),
        1 => {
            Equipment::menu(player);
            main(player);
        }
        2 => finances(player),
        3 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

pub fn mundane_inventory(player: &mut Player) {
    page_header("Items Inventory", HeaderSubtext::None);

    ItemInventory::print_shop(player);

    press_enter_to_continue();
    main(player);
}

pub fn finances(player: &mut Player) {
    page_header("Finances", HeaderSubtext::None);

    player.bank.print_inventory();

    press_enter_to_continue();
    main(player);
}
