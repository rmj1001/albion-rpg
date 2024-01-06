use albion_termrpg::lib::{
    terminal,
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn menu(user: &mut UserProfile) {
    loop {
        page_header(
            &format!("Game Menu (user: {})", user.username),
            Some(
                "Type the menu item code (ex. c3) and press ENTER/RETURN to perform that action.\n",
            ),
        );

        tui::sub_header("Combat");
        println!("c1. Wander the Realm");
        println!("c2. Enter the Stronghold");
        println!("\n");

        tui::sub_header("Economy");
        println!("e1. Work in the Guilds");
        println!("e2. The Bank");
        println!("e3. Trading Post");
        println!("e4. Weapons Shop");
        println!("e5. Armor Shop");
        println!("e6. Mystic Shop");
        println!("e7. Max's Shop");

        tui::sub_header("Profile");
        println!("p1. Inventory");
        println!("p2. Hall of Records");

        println!("\n");
        if user.is_developer {
            println!("d1. Developer Menu");
        }
        println!("n1. Settings");
        println!("n2. Logout");
        println!("n3. Exit Game\n");

        let choice = tui::dialogue::prompt_input("Enter Menu Code").to_lowercase();

        match &choice[..] {
            // Combat
            "c1" => crate::menus::game::combat::c1_the_stronghold::main(user),
            "c2" => crate::menus::game::combat::c2_wander_realm::main(user),

            // Economy
            "e1" => crate::menus::game::economy::e1_the_guilds::main(user),
            "e2" => crate::menus::game::economy::e2_the_bank::main(user),
            "e3" => crate::menus::game::economy::e3_trading_post::main(user),
            "e4" => crate::menus::game::economy::e4_weapons_shop::main(user),
            "e5" => crate::menus::game::economy::e5_armor_shop::main(user),
            "e6" => crate::menus::game::economy::e6_mystic_shop::main(user),
            "e7" => crate::menus::game::economy::e7_max_shop::main(user),

            // Profile
            "p1" => crate::menus::game::profile::p1_inventory::main(user),
            "p2" => crate::menus::game::profile::p2_hall_of_records::main(user),
            "n1" => crate::menus::game::profile::n1_settings::menu(user),
            "n2" => {
                user.save_profile();
                crate::menus::accounts::main::menu();
            }
            "n3" => {
                user.save_profile();
                terminal::exit();
            }

            // Developer Mode
            "d1" => {
                if user.is_developer {
                    crate::menus::game::profile::d1_developer_menu::main(user);
                } else {
                    tui::invalid_input(None);
                    continue;
                }
            }

            wrong_input => {
                tui::invalid_input(Some(wrong_input));
                continue;
            }
        }
    }
}
