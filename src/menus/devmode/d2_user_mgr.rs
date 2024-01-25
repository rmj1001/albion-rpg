use crate::{
    misc::{
        files,
        input::{confirm, select_from_str_array, select_from_vector},
        messages::{self, *},
        tui::{self, page_header, press_enter_to_continue, HeaderSubtext},
    },
    user::profile::UserProfile,
};

pub fn main(user: &mut UserProfile) {
    page_header("Developer Mode - User Manager", HeaderSubtext::Keyboard);

    let choice1 = select_from_str_array(
        &["1. List Users", "2. Delete User", "3. View User File", "NAV: Go Back"],
        None,
    );

    match choice1 {
        0 => list_users(user),
        1 => delete_users(user),
        2 => view_user(user),
        3 => super::d1_developer_menu::main(user),
        _ => out_of_bounds(),
    }
}

fn list_users(user: &mut UserProfile) {
    page_header("Developer Mode - User Manager", HeaderSubtext::None);

    let profiles: Vec<String> = files::list_all();

    for profile_string in &profiles {
        println!("- {}", profile_string);
    }

    println!();
    tui::press_enter_to_continue();

    main(user);
}

fn delete_users(user: &mut UserProfile) {
    page_header("Developer Mode - User Manager", HeaderSubtext::Keyboard);

    let profiles = files::list_all();
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
                main(user);
            }

            if *profile_string == user.settings.username {
                UserProfile::delete_from_username(&user.settings.username);

                page_header("Developer Mode - User Manager", HeaderSubtext::None);
                success_msg("Current profile deleted. Logging out.");

                crate::menus::accounts::main();
            }

            UserProfile::delete_from_username(profile_string);

            page_header("Developer Mode - User Manager", HeaderSubtext::None);
            success_msg(format!("Profile '{}' deleted.", profile_string));

            main(user);
        }

        None => out_of_bounds(),
    }
}

fn view_user(user: &mut UserProfile) {
    page_header("Developer Mode - User Manager - Data Viewer", HeaderSubtext::None);
    let choice = select_from_vector(files::list_all(), Some("Select a user to view"));

    let profiles = files::list_all();
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let profile_result = UserProfile::retrieve(profile_string);

            match profile_result {
                Ok(profile) => {
                    let pretty_string_result = crate::misc::config_encoding::serialize_user(user);
                    let mut pretty_string: String = String::new();

                    match pretty_string_result {
                        Ok(data) => pretty_string = data,
                        Err(message) => {
                            messages::failure(format!("{}", message));
                        }
                    }

                    page_header(
                        format!("User Profile - {}", profile.settings.username),
                        HeaderSubtext::None,
                    );

                    println!("{}\n", pretty_string);

                    press_enter_to_continue();
                    main(user);
                }
                Err(message) => {
                    failure(message);
                    main(user);
                }
            }
        }
        None => out_of_bounds(),
    }

    main(user);
}
