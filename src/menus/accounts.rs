use crate::utils::{input::select_from_str_array, messages::*, terminal::*, tui::*};

pub fn main() {
    clearscr();
    page_header("Accounts Menu", HeaderSubtext::Keyboard);

    let main_menu_options = vec!["1. Login", "2. Register", "3. Exit"];

    let chosen_option = select_from_str_array(&main_menu_options, None);

    match chosen_option {
        0 => crate::menus::login::main(),
        1 => crate::menus::register::main(),
        2 => exit(None),
        _ => out_of_bounds(),
    }
}
