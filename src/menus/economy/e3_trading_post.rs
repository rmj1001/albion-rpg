use crate::{
    lib::tui::{page_header, press_enter_to_continue},
    user::profile::UserProfile,
};

// TODO: Trading Post
pub fn main(user: &mut UserProfile) {
    page_header("Trading Post", crate::lib::tui::HeaderSubtext::None);

    user.inventory.print_table();

    press_enter_to_continue();

    crate::menus::game_menu::main(user);
}
