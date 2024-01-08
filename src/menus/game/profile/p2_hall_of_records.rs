use albion_termrpg::lib::{
    input::selector,
    tui::{page_header, press_enter_to_continue, HeaderInstructions},
    user::xp::XP,
    user::{achievements::Achievements, profile::UserProfile},
};

fn print_xp(xp: u32) {
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
        2 => crate::menus::game::main::menu(user),
        _ => panic!("Dialoguer picked array index out of bounds"),
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
    #[allow(unused_variables)]
    let achievements: &Achievements = &user.achievements;

    page_header("Hall of Records - Achievements", HeaderInstructions::None);

    // TODO: List achievements here

    press_enter_to_continue();
    main(user);
}
