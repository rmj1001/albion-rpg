use crate::{
    data::settings::Settings,
    utils::{
        input::{confirm, select_from_str_array},
        messages::*,
        tui::{page_header, HeaderSubtext},
    },
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Developer Mode", HeaderSubtext::Keyboard);

    let choice = select_from_str_array(
        &[
            "1. Throw a panic",
            "2. Manipulate Inventory",
            "3. Manipulate XP",
            "4. Manipulate Banks",
            "5. Manage Player Profiles",
            "6. Disable developer mode",
            "NAV: Go Back",
        ],
        None,
    );

    match choice {
        0 => panic!("This is a panic!"),
        1 => super::d4_inventory_mgr::main(player),
        2 => super::d3_xp_mgr::main(player),
        3 => super::d5_bank_mgr::main(player),
        4 => super::d2_user_mgr::main(player),
        5 => disable_developer_mode(player),
        6 => {
            player.save();
            crate::menus::game_menu::main(player);
        }
        _ => out_of_bounds(),
    }
}

pub fn disable_developer_mode(player: &mut Player) {
    page_header("Developer Mode", HeaderSubtext::None);

    if !player.settings.developer {
        failure("Developer mode is already disabled.");
        crate::menus::game_menu::main(player);
    }

    let disable_dev_mode = confirm("Are you sure you want to disable developer mode?");

    if !disable_dev_mode {
        cancelling();
        main(player);
    }

    Settings::toggle_developer(player);

    crate::menus::game_menu::main(player);
}
