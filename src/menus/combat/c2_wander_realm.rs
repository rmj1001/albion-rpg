use crate::{
    lib::{
        messages::failure,
        tui::{self, page_header, HeaderSubtext},
    },
    user::profile::UserProfile,
};

// TODO: Wandering the Realm
pub fn main(user: &mut UserProfile) {
    page_header("Wandering the Realm", HeaderSubtext::None);

    println!("You are wandering the realm...");
    tui::sleep(3);

    failure("This is not yet implemented.");

    crate::menus::game_menu::main(user);
}
