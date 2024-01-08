use albion_termrpg::lib::{
    input::{out_of_bounds, selector},
    terminal::*,
    tui::*,
};

pub fn menu() {
    clear();
    page_header("Accounts Menu", HeaderInstructions::Keyboard);

    let main_menu_options = vec!["Login", "Register", "Exit"];

    let chosen_option = selector(&main_menu_options, 0, None);

    match chosen_option {
        0 => crate::menus::accounts::login::menu(),
        1 => crate::menus::accounts::register::menu(),
        2 => exit(),
        _ => out_of_bounds(None),
    }
}
