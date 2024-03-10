use crate::{data::achievements::Achievements, prelude::*};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Hall of Records", HeaderSubtext::Keyboard);

    let menu_option = select_from_str_array(&["1. XP/Levels", "2. Achievements", "NAV: Go Back"], None);

    match menu_option {
        0 => xp(player),
        1 => achievements(player),
        2 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

pub fn xp(player: &mut Player) {
    page_header("Hall of Records - XP/Levels", HeaderSubtext::None);

    player.xp.table();

    press_enter_to_continue();
    main(player);
}

pub fn achievements(player: &mut Player) {
    // These check to see if new achievements have been earned before
    // printing them
    Achievements::check(player);

    // Save the profile so new achievements are written to disk
    player.save();

    // Print out all achievements
    page_header("Hall of Records - Achievements", HeaderSubtext::None);

    player.achievements.table();

    press_enter_to_continue();
    main(player);
}
