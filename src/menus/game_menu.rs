use crate::{
    combat::battle::Battle,
    data::{achievements::Achievements, inventory::equipment::Equipment, settings::Settings},
    prelude::{confirm, exit, invalid_input, page_header, pause, prompt, sleep, success, Instructions},
};

use crate::data::player::Player;

fn print_menu(player: &mut Player) {
    page_header(
        format!("Game Menu (Player: {})", player.settings.username),
        &Instructions::TypeCode,
    );

    println!("#------- Combat ------#");
    println!("1. Wander the Realm");
    println!("2. Enter the Stronghold");
    println!();

    println!("#------- Economy -----#");
    println!("3. The Guilds");
    println!("4. The Bank");
    println!("5. Trading Post");
    println!("6. Weapons Shop");
    println!("7. Armor Shop");
    println!();

    println!("#------ Profile ------#");
    println!("8. Inventory");
    println!("9. Hall of Records");
    println!();

    println!("#----- Settings ------#");
    if player.settings.developer {
        println!("96. Developer Menu");
    }

    println!("97. Settings");
    println!("98. Save Game");
    println!("99. Logout");
    println!();
}

pub fn main(player: &mut Player) {
    // Check for achievements at login to keep the player file up to date
    Achievements::check(player);

    // Update armor equipment status
    Equipment::check_equipment_ownership(player);

    print_menu(player);

    let choice = prompt("Enter Menu Code").to_lowercase();

    match &choice[..] {
        // Combat
        "1" | "wander the realm" => {
            Battle::new("Wandering the Wild", "You are wandering the realm...", player, 0, None).start();
        }
        "2" | "enter the stronghold" => {
            page_header("The Stronghold", &Instructions::None);

            let enter_stronghold =
                confirm("Are you sure you want to enter the stronghold? You must win many hard battles.");

            if enter_stronghold {
                Battle::new(
                    "The Stronghold",
                    "You delve into the stronghold...",
                    player,
                    50,
                    Some(exit_stronghold),
                )
                .start();
            }

            main(player);
        }

        // Economy
        "3" | "the guilds" => crate::menus::economy::e1_the_guilds::main(player),
        "4" | "the bank" => crate::menus::economy::e2_the_bank::main(player),
        "5" | "trading post" => crate::menus::economy::e3_trading_post::main(player),
        "6" | "weapons shop" => crate::menus::economy::e4_weapons_shop::main(player),
        "7" | "armor shop" => crate::menus::economy::e5_armor_shop::main(player),

        // Profile
        "8" | "inventory" => crate::menus::profile::p1_inventory::main(player),
        "9" | "hall of records" => crate::menus::profile::p2_hall_of_records::main(player),
        "97" | "settings" => crate::menus::profile::n1_settings::main(player),
        "98" | "save game" | "save" => {
            page_header("Saving Game", &Instructions::None);
            println!("\nSaving game...");
            sleep(2);

            player.save();
            success(None);

            main(player);
        }
        "99" | "logout" => {
            player.save();

            page_header("Accounts Menu", &Instructions::None);
            println!("\nLogging out...");
            sleep(2);

            crate::menus::accounts::main();
        }

        "exit" => {
            exit(Some(player));
        }

        "3.141592" => {
            page_header("Developer Mode", &Instructions::None);
            Settings::toggle_developer(player);
            main(player);
        }

        misc => match misc {
            "96" | "developer" => {
                if player.settings.developer {
                    crate::menus::devmode::d1_developer_menu::main(player);
                } else {
                    invalid_input(Some(misc), None, true);
                    main(player);
                }
            }
            _ => {
                invalid_input(Some(misc), None, true);
                main(player);
            }
        },
    }
}

fn exit_stronghold(player: &mut Player) {
    page_header("The Stronghold", &Instructions::None);

    println!("\nYou have successfully completed the stronghold and won the game! Congratulations!");
    player.achievements.stronghold_defeated = true;
    player.save();

    pause();
    main(player);
}
