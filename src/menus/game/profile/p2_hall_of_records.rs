use crate::lib::{
    input::{out_of_bounds, selector},
    tui::{page_header, press_enter_to_continue, HeaderInstructions},
};

use crate::user::{achievements::Achievements, profile::UserProfile, xp::XP};

fn print_xp(xp: usize) {
    println!("XP: {}", xp);
    println!("Level: {}", XP::level(xp));
    println!();
}

pub fn main(user: &mut UserProfile) {
    page_header("Hall of Records", HeaderInstructions::Keyboard);

    let menu_option = selector(
        &["1. XP/Levels", "2. Achievements", "NAV: Go Back"],
        0,
        None,
    );

    match menu_option {
        0 => xp(user),
        1 => achievements(user),
        2 => crate::menus::game::main::main(user),
        _ => out_of_bounds(None),
    }
}

pub fn xp(user: &mut UserProfile) {
    let xp: &XP = &user.xp;

    page_header("Hall of Records - XP/Levels", HeaderInstructions::None);

    println!("# Profile");
    print_xp(xp.total_xp());

    println!("# Combat");
    print_xp(xp.combat);

    println!("# Fishing");
    print_xp(xp.fishing);

    println!("# Cooking");
    print_xp(xp.cooking);

    println!("# Woodcutting");
    print_xp(xp.woodcutting);

    println!("# Mining");
    print_xp(xp.mining);

    println!("# Smithing");
    print_xp(xp.smithing);

    println!("# Thieving");
    print_xp(xp.thieving);

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
    let achievements: &Achievements = &user.achievements;

    page_header("Hall of Records - Achievements", HeaderInstructions::None);

    println!("Monsters Killed: {}\n", achievements.monsters_killed);
    println!(
        "1,000,000 Gold? {}\n",
        pretty_bool(achievements.earned_million_gold)
    );
    println!(
        "Defeated Stronghold? {}\n",
        pretty_bool(achievements.stronghold_defeated)
    );
    println!(
        "Reached Level 100? {}\n",
        pretty_bool(achievements.level_100_reached)
    );
    println!(
        "Hacked the Game? {}\n",
        pretty_bool(achievements.hacked_the_game)
    );

    press_enter_to_continue();
    main(user);
}

fn pretty_bool(flag: bool) -> &'static str {
    if flag {
        "Yes!"
    } else {
        "No."
    }
}
