pub mod menus {
    pub mod accounts {
        pub mod login;
        pub mod main;
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
            pub mod e7_max_shop;
        }

        pub mod profile {
            pub mod d1_developer_menu;
            pub mod n1_settings;
            pub mod p1_inventory;
            pub mod p2_hall_of_records;
        }

        pub mod main;
    }
}

fn main() {
    anglandia_text_rpg::lib::os::detect_os();
    menus::accounts::main::menu();
}
