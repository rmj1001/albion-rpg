use albion_termrpg::lib::{
    tui::{page_header, press_enter_to_continue},
    user_profile::{UserProfile, XP},
};

fn print_xp(xp: u32) {
    println!("XP: {}", xp);
    println!("Level: {}", XP::level(xp));
    println!();
}

pub fn main(user: &mut UserProfile) {
    let xp: &XP = &user.xp;

    page_header("Hall of Records", None);

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
    crate::menus::game::main::menu(user);
}
