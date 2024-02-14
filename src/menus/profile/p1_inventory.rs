use crate::utils::{
    input::select_from_str_array,
    messages::*,
    tui::{page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::player::profile::UserProfile;

pub fn main(player: &mut UserProfile) {
    page_header("Inventory Viewer", HeaderSubtext::None);

    let menu_option = select_from_str_array(
        &["1. Items", "2. Weapons", "3. Armor", "4. Finances", "NAV: Go Back"],
        None,
    );

    match menu_option {
        0 => mundane_inventory(player),
        1 => weapons_inventory(player),
        2 => armor_inventory(player),
        3 => finances(player),
        4 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

pub fn mundane_inventory(player: &mut UserProfile) {
    page_header("Items Inventory", HeaderSubtext::None);

    player.inventory.print_table();

    press_enter_to_continue();
    main(player);
}

pub fn weapons_inventory(player: &mut UserProfile) {
    page_header("Weapons Inventory", HeaderSubtext::None);

    player.weapons.print_table();

    // TODO: Equip/Unequip weapons

    press_enter_to_continue();
    main(player);
}

pub fn armor_inventory(player: &mut UserProfile) {
    page_header("Armor Inventory", HeaderSubtext::None);

    player.armor.print_table();

    // TODO: Equip/Unequip armor

    press_enter_to_continue();
    main(player);
}

pub fn finances(player: &mut UserProfile) {
    page_header("Finances", HeaderSubtext::None);

    player.bank.print_table();

    press_enter_to_continue();
    main(player);
}
