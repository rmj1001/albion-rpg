use crate::{data::player::Player, prelude::*};

pub fn main(player: &mut Player) {
    page_header("Developer Mode - Player Manager", HeaderSubtext::Keyboard);

    let choice1 = select_from_str_array(
        &[
            "1. List Players",
            "2. Delete Player",
            "3. View Player File",
            "NAV: Go Back",
        ],
        None,
    );

    match choice1 {
        0 => list_users(player),
        1 => delete_users(player),
        2 => view_user(player),
        3 => super::d1_developer_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn list_users(player: &mut Player) {
    page_header("Developer Mode - Player Manager", HeaderSubtext::None);

    let profiles: Vec<String> = file_handler::list_all_profiles();

    profiles.iter().for_each(|profile_string| {
        println!("- {}", profile_string);
    });

    println!();
    press_enter_to_continue();

    main(player);
}

fn delete_users(player: &mut Player) {
    page_header("Developer Mode - Player Manager", HeaderSubtext::Keyboard);

    let profiles = file_handler::list_all_profiles();
    let choice = select_from_vector(profiles.clone(), Some("Select a profile to delete"));
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let delete_profile = confirm(&format!(
                "Are you sure you want to delete profile '{}'?",
                profile_string
            ));

            if !delete_profile {
                cancelling();
                main(player);
            }

            if *profile_string == player.settings.username {
                Player::delete_from_username(&player.settings.username);

                page_header("Developer Mode - Player Manager", HeaderSubtext::None);
                success_msg("Current profile deleted. Logging out.");

                crate::menus::accounts::main();
            }

            Player::delete_from_username(profile_string);

            page_header("Developer Mode - Player Manager", HeaderSubtext::None);
            success_msg(format!("Profile '{}' deleted.", profile_string));

            main(player);
        }

        None => out_of_bounds(),
    }
}

fn view_user(player: &mut Player) {
    page_header("Developer Mode - Player Manager - Data Viewer", HeaderSubtext::None);
    let choice = select_from_vector(file_handler::list_all_profiles(), Some("Select a player to view"));

    let profiles = file_handler::list_all_profiles();
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let profile_result = Player::get_from_username(profile_string);

            match profile_result {
                Ok(profile) => {
                    Player::paged_viewer(&profile);
                    main(player);
                }
                Err(message) => {
                    message.failure();
                    main(player);
                }
            }
        }
        None => out_of_bounds(),
    }

    main(player);
}
