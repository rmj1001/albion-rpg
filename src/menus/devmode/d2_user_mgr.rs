use crate::{
    data::player::Player,
    prelude::{all_profiles, cancel, confirm, failure, page_header, pause, select, success, unreachable, Instructions},
};

pub fn main(player: &mut Player) {
    loop {
        page_header("Developer Mode - Player Manager", &Instructions::Keyboard);

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
            0 => list_users(),
            1 => delete_users(player),
            2 => view_user(),
            3 => super::d1_developer_menu::main(player),
            _ => unreachable(),
        }
    }
}

fn list_users() {
    page_header("Developer Mode - Player Manager", &Instructions::None);

    let profiles: Vec<String> = all_profiles();

    for profile_string in &profiles {
        println!("- {profile_string}");
    }

    println!();
    pause();
}

fn delete_users(player: &mut Player) {
    page_header("Developer Mode - Player Manager", &Instructions::Keyboard);

    let profiles = all_profiles();
    let choice = select(&profiles, Some("Select a profile to delete"));
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let delete_profile = confirm(&format!("Are you sure you want to delete profile '{profile_string}'?"));

            if !delete_profile {
                cancel(None);
                return;
            }

            if *profile_string == player.settings.username {
                page_header("Developer Mode - Player Manager", &Instructions::None);

                match Player::delete_from(&player.settings.username) {
                    Ok(()) => success(Some("Current profile deleted. Logging out.")),
                    Err(error) => failure(&error.to_string()),
                }

                crate::menus::accounts::main();
            }

            page_header("Developer Mode - Player Manager", &Instructions::None);

            match Player::delete_from(profile_string) {
                Ok(()) => success(Some(&format!("Profile '{profile_string}' deleted."))),
                Err(error) => failure(&error.to_string()),
            }
        }

        None => unreachable(),
    }
}

fn view_user() {
    page_header("Developer Mode - Player Manager - Data Viewer", &Instructions::None);
    let choice = select(&all_profiles(), Some("Select a player to view"));

    let profiles = all_profiles();
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let profile_result = Player::get(profile_string);

            match profile_result {
                Ok(profile) => {
                    Player::paginate(&profile);
                }
                Err(message) => {
                    message.print(true);
                }
            }
        }
        None => unreachable(),
    }
}
