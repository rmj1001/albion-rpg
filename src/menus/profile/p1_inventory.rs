use crate::{
    data::inventory::{equipment::Equipment, items::ItemInventory},
    prelude::*,
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Inventory Viewer", &Instructions::None);

    let menu_option = select(&["1. Items", "2. Equipment", "3. Finances", "NAV: Go Back"], None);

    match menu_option {
        0 => mundane_inventory(player),
        1 => {
            Equipment::menu(player);
            main(player);
        }
        2 => finances(player),
        3 => crate::menus::game_menu::main(player),
        _ => unreachable(),
    }
}

pub fn mundane_inventory(player: &mut Player) {
    page_header("Items Inventory", &Instructions::None);

    ItemInventory::shop_table(player);

    pause();
    main(player);
}

pub fn finances(player: &mut Player) {
    page_header("Finances", &Instructions::None);

    player.bank.table();

    pause();
    main(player);
}
