use crate::utils::{
    input::select_from_str_array,
    messages::*,
    tui::{page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::player::profile::UserProfile;

pub fn main(player: &mut UserProfile) {
    page_header("Hall of Records", HeaderSubtext::Keyboard);

    let menu_option = select_from_str_array(&["1. XP/Levels", "2. Achievements", "NAV: Go Back"], None);

    match menu_option {
        0 => xp(player),
        1 => achievements(player),
        2 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

pub fn xp(player: &mut UserProfile) {
    page_header("Hall of Records - XP/Levels", HeaderSubtext::None);

    player.xp.print_table();

    press_enter_to_continue();
    main(player);
}

pub fn achievements(player: &mut UserProfile) {
    // These check to see if new achievements have been earned before
    // printing them
    if !player.achievements.hacked_the_game && player.settings.developer {
        player.achievements.hacked_the_game = true;
    }

    if !player.achievements.level_100_reached && player.xp.profile_level() >= 100 {
        player.achievements.level_100_reached = true;
    }

    if !player.achievements.earned_million_gold && player.bank.wallet >= 1_000_000 {
        player.achievements.earned_million_gold = true;
    }

    // Save the profile so new achievements are written to disk
    player.save();

    // Print out all achievements
    page_header("Hall of Records - Achievements", HeaderSubtext::None);

    player.achievements.print_table();

    press_enter_to_continue();
    main(player);
}
