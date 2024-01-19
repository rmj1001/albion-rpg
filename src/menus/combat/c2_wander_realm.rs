use crate::{
    lib::tui::{page_header, press_enter_to_continue, HeaderSubtext},
    user::profile::UserProfile,
};

// TODO: Wandering the Realm
pub fn main(user: &mut UserProfile) {
    page_header("Wandering the Realm", HeaderSubtext::None);

    println!("This is not yet implemented.");
    press_enter_to_continue();

    crate::menus::game_menu::main(user);
}
