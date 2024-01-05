pub mod menus {
    pub mod start {
        pub mod login;
        pub mod main_menu;
        pub mod register;
    }
}

fn main() {
    menus::start::main_menu::menu()
}
