pub mod menus {
    pub mod accounts {
        pub mod login;
        pub mod main;
        pub mod register;
    }

    pub mod game {
        pub mod main;
    }
}

fn main() {
    anglandia_text_rpg::lib::os::detect_os();
    menus::accounts::main::menu();
}
