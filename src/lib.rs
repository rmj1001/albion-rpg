#![warn(clippy::pedantic, clippy::wildcard_imports)]
#![allow(
    unused_assignments,
    clippy::new_without_default,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools
)]
#![doc = include_str!("../README.md")]

pub mod utils {
    pub mod crypt;
    pub mod error;
    pub mod files;
    pub mod input;
    pub mod math;
    pub mod messages;
    pub mod terminal;
    pub mod tui;
}

pub mod data {
    pub mod inventory {
        pub mod armor;
        pub mod bank;
        pub mod equipment;
        pub mod items;
        pub mod weapons;
    }
    pub mod achievements;
    pub mod guilds;
    pub mod health;
    pub mod player;
    pub mod settings;
    pub mod xp;
}

pub mod menus {
    pub mod devmode {
        pub mod d1_developer_menu;
        pub mod d2_user_mgr;
        pub mod d3_xp_mgr;
        pub mod d4_inventory_mgr;
        pub mod d5_bank_mgr;
    }

    pub mod economy {
        pub mod e1_the_guilds;
        pub mod e2_the_bank;
        pub mod e3_trading_post;
        pub mod e4_weapons_shop;
        pub mod e5_armor_shop;
    }

    pub mod profile {
        pub mod n1_settings;
        pub mod p1_inventory;
        pub mod p2_hall_of_records;
    }

    pub mod accounts;
    pub mod game_menu;
    pub mod login;
    pub mod register;
}

pub mod combat {

    pub mod battle;
    pub mod enemy;
    pub mod inventory;
}

pub mod prelude {
    pub use crate::utils::{
        crypt::*,
        error::{self, check_debug_mode, unreachable},
        files::*,
        input::*,
        math::*,
        messages::*,
        terminal::*,
        tui::*,
    };
}
