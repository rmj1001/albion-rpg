use anglandia_text_rpg::lib::terminal::*;
use anglandia_text_rpg::lib::tui::*;

pub fn menu() {
    loop {
        clear_screen();
        page_header("Accounts Menu", None);

        let main_menu_options = vec!["Login", "Register", "NAV: Exit"];

        let chosen_option = dialogue::selector(&main_menu_options, 0, None);

        match chosen_option {
            0 => crate::menus::accounts::login::menu(),
            1 => crate::menus::accounts::register::menu(),
            2 => exit(),
            _ => {
                panic!("Invalid option chosen.");
            }
        }
    }
}
