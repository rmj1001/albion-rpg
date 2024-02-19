use crate::{
    data::player::Player,
    utils::{
        files,
        input::{confirm, select_from_str_array, select_from_vector},
        messages::{self, *},
        tui::{self, page_header, paginate_string, press_enter_to_continue, HeaderSubtext},
    },
};

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

    let profiles: Vec<String> = files::handler::list_all_profiles();

    for profile_string in &profiles {
        println!("- {}", profile_string);
    }

    println!();
    tui::press_enter_to_continue();

    main(player);
}

fn delete_users(player: &mut Player) {
    page_header("Developer Mode - Player Manager", HeaderSubtext::Keyboard);

    let profiles = files::handler::list_all_profiles();
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
    let choice = select_from_vector(files::handler::list_all_profiles(), Some("Select a player to view"));

    let profiles = files::handler::list_all_profiles();
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let profile_result = Player::get_from_username(profile_string);

            match profile_result {
                Ok(profile) => {
                    let pretty_string_result = crate::utils::files::encoding::encode(player);
                    let mut pretty_string: String = String::new();
                    let mut paginated_file: Vec<String> = Vec::new();

                    match pretty_string_result {
                        Ok(data) => pretty_string = data,
                        Err(message) => {
                            messages::failure(format!("{}", message));
                        }
                    }

                    paginated_file = paginate_string(pretty_string, 20);
                    let mut page_number: usize = 1;
                    let total_pages = paginated_file.len();

                    for page in paginated_file {
                        page_header(
                            format!(
                                "Player Profile - {} - Page {}/{}",
                                profile.settings.username, page_number, total_pages
                            ),
                            HeaderSubtext::None,
                        );

                        println!("{}\n", page);
                        press_enter_to_continue();

                        page_number += 1;
                    }

                    main(player);
                }
                Err(message) => {
                    failure(message);
                    main(player);
                }
            }
        }
        None => out_of_bounds(),
    }

    main(player);
}
