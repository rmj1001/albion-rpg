use crate::{
    misc::tui::{page_header, press_enter_to_continue, HeaderSubtext},
    user::profile::UserProfile,
};

// TODO: The Stronghold: Gauntlet of 50 consecutive battles. If won then the game is complete.
pub fn main(user: &mut UserProfile) {
    page_header("The Stronghold", HeaderSubtext::None);

    println!("This is not yet implemented.");
    press_enter_to_continue();

    crate::menus::game_menu::main(user);
}
