use albion_termrpg::lib::{
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn main(user: &mut UserProfile) {
    page_header(
        "Developer Settings",
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    let choice = tui::dialogue::selector(
        &["1. Throw a panic", "NAV: Go Back to Main Menu"],
        0,
        Some(""),
    );

    match choice {
        0 => panic!("This is a panic!"),
        1 => crate::menus::game::main::menu(user),
        _ => panic!("Dialogue picked option out of bounds"),
    }
}
