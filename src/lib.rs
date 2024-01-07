#![allow(unused_assignments, clippy::new_without_default)]

pub mod lib {
    pub mod os;
    pub mod terminal;
    pub mod tui;
    pub mod user {
        pub mod armor;
        pub mod bank;
        pub mod inventory;
        pub mod profile;
        pub mod weapons;
        pub mod xp;
    }
}
