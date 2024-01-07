use albion_termrpg::lib::terminal::*;
use albion_termrpg::lib::tui::*;

pub fn menu() {
    clear();
    page_header(
        "Accounts Menu",
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    let main_menu_options = vec!["Login", "Register", "Exit"];

    let chosen_option = dialogue::selector(&main_menu_options, 0, Some(""));

    match chosen_option {
        0 => crate::menus::accounts::login::menu(),
        1 => crate::menus::accounts::register::menu(),
        2 => exit(),
        _ => {
            panic!("Invalid option chosen.");
        }
    }
}
