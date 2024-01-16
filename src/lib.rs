#![allow(unused_assignments, clippy::new_without_default)]

pub mod lib {
    pub mod crypt;
    pub mod input;
    pub mod math;
    pub mod os;
    pub mod terminal;
    pub mod tui;
}

pub mod user {
    pub mod achievements;
    pub mod armor;
    pub mod bank;
    pub mod combat;
    pub mod guilds;
    pub mod inventory;
    pub mod profile;
    pub mod settings;
    pub mod weapons;
    pub mod xp;
}

pub mod menus {
    pub mod accounts {
        pub mod accounts;
        pub mod login;
        pub mod register;
    }

    pub mod game {
        pub mod combat {
            pub mod c1_the_stronghold;
            pub mod c2_wander_realm;
        }

        pub mod economy {
            pub mod e1_the_guilds;
            pub mod e2_the_bank;
            pub mod e3_trading_post;
            pub mod e4_weapons_shop;
            pub mod e5_armor_shop;
            pub mod e6_mystic_shop;
            pub mod e7_celestial_shop;
        }

        pub mod profile {
            pub mod d1_developer_menu;
            pub mod n1_settings;
            pub mod p1_inventory;
            pub mod p2_hall_of_records;
        }

        pub mod game_menu;
    }
}
