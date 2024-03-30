use crate::{data::player::Player, prelude::*};

pub fn main(player: &mut Player) {
    page_header("Developer Mode - Player Manager", Instructions::Keyboard);

    let choice1 = select(
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
    page_header("Developer Mode - Player Manager", Instructions::None);

    let profiles: Vec<String> = all_profiles();

    profiles.iter().for_each(|profile_string| {
        println!("- {}", profile_string);
    });

    println!();
    pause();

    main(player);
}

fn delete_users(player: &mut Player) {
    page_header("Developer Mode - Player Manager", Instructions::Keyboard);

    let profiles = all_profiles();
    let choice = select(&profiles, Some("Select a profile to delete"));
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let delete_profile = confirm(&format!(
                "Are you sure you want to delete profile '{}'?",
                profile_string
            ));

            if !delete_profile {
                cancel(None);
                main(player);
            }

            if *profile_string == player.settings.username {
                page_header("Developer Mode - Player Manager", Instructions::None);

                match Player::delete_from(&player.settings.username) {
                    Ok(_) => success(Some("Current profile deleted. Logging out.")),
                    Err(error) => failure(&error.to_string()),
                }

                crate::menus::accounts::main();
            }

            page_header("Developer Mode - Player Manager", Instructions::None);

            match Player::delete_from(profile_string) {
                Ok(_) => success(Some(&format!("Profile '{}' deleted.", profile_string))),
                Err(error) => failure(&error.to_string()),
            }

            main(player);
        }

        None => out_of_bounds(),
    }
}

fn view_user(player: &mut Player) {
    page_header("Developer Mode - Player Manager - Data Viewer", Instructions::None);
    let choice = select(&all_profiles(), Some("Select a player to view"));

    let profiles = all_profiles();
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let profile_result = Player::get(profile_string);

            match profile_result {
                Ok(profile) => {
                    Player::paginate(&profile);
                    main(player);
                }
                Err(message) => {
                    message.print(true);
                    main(player);
                }
            }
        }
        None => out_of_bounds(),
    }

    main(player);
}
