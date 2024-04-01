use crate::prelude::{clearscr, exit, page_header, select, unreachable, Instructions};

pub fn main() {
    clearscr();
    page_header("Accounts Menu", &Instructions::Keyboard);

    let chosen_option = select(&["1. Login", "2. Register", "3. Exit"], None);

    match chosen_option {
        0 => crate::menus::login::main(),
        1 => crate::menus::register::main(),
        2 => exit(None),
        _ => unreachable(),
    }
}
