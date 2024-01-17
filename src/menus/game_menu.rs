use crate::lib::{
    input::{self, prompt_arrow},
    terminal,
    tui::{self, page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header(
        &format!("Game Menu (user: {})", user.settings.username),
        tui::HeaderSubtext::EnterCode,
    );

    tui::small_header("Combat", HeaderSubtext::None);
    println!("c1. Wander the Realm");
    println!("c2. Enter the Stronghold");
    println!("\n");

    tui::small_header("Economy", HeaderSubtext::None);
    println!("e1. Work in the Guilds");
    println!("e2. The Bank");
    println!("e3. Trading Post");
    println!("e4. Weapons Shop");
    println!("e5. Armor Shop");
    println!("\n");

    tui::small_header("Profile", HeaderSubtext::None);
    println!("p1. Inventory");
    println!("p2. Hall of Records");
    println!("\n");

    if user.settings.developer {
        println!("d1. Developer Menu");
    }

    println!("n1. Settings");
    println!("n2. Logout");
    println!("n3. Exit Game\n");

    let choice = prompt_arrow("Enter Menu Code").to_lowercase();

    match &choice[..] {
        // Combat
        "c1" => crate::menus::combat::c1_the_stronghold::main(user),
        "c2" => crate::menus::combat::c2_wander_realm::main(user),

        // Economy
        "e1" => crate::menus::economy::e1_the_guilds::main(user),
        "e2" => crate::menus::economy::e2_the_bank::main(user),
        "e3" => crate::menus::economy::e3_trading_post::main(user),
        "e4" => crate::menus::economy::e4_weapons_shop::main(user),
        "e5" => crate::menus::economy::e5_armor_shop::main(user),

        // Profile
        "p1" => crate::menus::profile::p1_inventory::main(user),
        "p2" => crate::menus::profile::p2_hall_of_records::main(user),
        "n1" => crate::menus::profile::n1_settings::main(user),
        "n2" => {
            user.save();
            crate::menus::accounts::main();
        }
        "n3" => {
            user.save();
            terminal::exit();
        }

        // Developer Mode
        "d1" => {
            if user.settings.developer {
                crate::menus::profile::d1_developer_menu::main(user);
            } else {
                input::invalid_input(None, None, true);
                main(user);
            }
        }

        "3.141592" => {
            if !user.settings.developer {
                page_header("Developer Mode", HeaderSubtext::None);
                user.achievements.hacked_the_game = true;
                user.settings.set_developer(None, true);

                println!("\nDeveloper mode enabled.");
                press_enter_to_continue();
                main(user);
            } else {
                super::profile::d1_developer_menu::disable_developer_mode(user);
            }
        }

        wrong_input => {
            input::invalid_input(Some(wrong_input), None, true);
            main(user);
        }
    }
}
