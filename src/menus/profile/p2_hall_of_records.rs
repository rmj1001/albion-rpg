use crate::lib::{
    input::select_from_str_array,
    messages::*,
    tui::{page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header("Hall of Records", HeaderSubtext::Keyboard);

    let menu_option =
        select_from_str_array(&["1. XP/Levels", "2. Achievements", "NAV: Go Back"], None);

    match menu_option {
        0 => xp(user),
        1 => achievements(user),
        2 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(),
    }
}

pub fn xp(user: &mut UserProfile) {
    page_header("Hall of Records - XP/Levels", HeaderSubtext::None);

    user.xp.print_table();

    press_enter_to_continue();
    main(user);
}

pub fn achievements(user: &mut UserProfile) {
    // These check to see if new achievements have been earned before
    // printing them
    if !user.achievements.hacked_the_game && user.settings.developer {
        user.achievements.hacked_the_game = true;
    }

    if !user.achievements.level_100_reached && user.xp.profile_level() >= 100 {
        user.achievements.level_100_reached = true;
    }

    if !user.achievements.earned_million_gold && user.bank.wallet >= 1_000_000 {
        user.achievements.earned_million_gold = true;
    }

    // Save the profile so new achievements are written to disk
    user.save();

    // Print out all achievements
    page_header("Hall of Records - Achievements", HeaderSubtext::None);

    user.achievements.print_table();

    press_enter_to_continue();
    main(user);
}
