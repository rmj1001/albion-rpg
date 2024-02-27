#![allow(unused_assignments, clippy::new_without_default)]

pub use utils::error::{DataError, InventoryError, MiscError, ProfileError, Result};

pub mod utils {
    pub mod crypt;
    pub mod error;
    pub mod files;
    pub mod input;
    pub mod math;
    pub mod messages;
    pub mod os;
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
