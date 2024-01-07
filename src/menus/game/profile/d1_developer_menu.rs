// TODO: Remove this once the functions below are implemented.
#![allow(unused_variables)]

use albion_termrpg::lib::{
    tui::{self, page_header},
    user::profile::UserProfile,
};

fn manage_user_profiles(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Profile Management",
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    // Listing profiles for printing or deletion
    let profiles: Vec<String> = UserProfile::list_all();

    let choice1 = tui::dialogue::selector(
        &["1. List Profiles", "2. Delete profiles", "NAV: Go Back"],
        0,
        Some(""),
    );

    match choice1 {
        // listing profiles
        0 => {
            page_header("Developer Mode - Profile Management", None);

            for profile_string in &profiles {
                println!("- {}", profile_string);
            }

            println!();
            tui::press_enter_to_continue();

            manage_user_profiles(user);
        }

        // deleting profiles
        1 => {
            page_header(
                "Developer Mode - Profile Management",
                Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
            );

            let choice = tui::dialogue::select_from_vector(
                profiles.clone(),
                0,
                Some("Select a profile to delete"),
            );

            let profile_choice = profiles.get(choice);

            match profile_choice {
                Some(profile_string) => {
                    match &tui::dialogue::prompt_input(&format!(
                        "Are you sure you want to delete profile '{}'? (y/n)",
                        profile_string
                    ))
                    .to_lowercase()[..]
                    {
                        "n" => {
                            println!("\nAborting.");
                            tui::press_enter_to_continue();

                            manage_user_profiles(user);
                        }
                        "no" => {
                            println!("\nAborting.");
                            tui::press_enter_to_continue();

                            manage_user_profiles(user);
                        }
                        "y" => {}
                        "yes" => {}

                        invalid_input => {
                            println!("\nInvalid input. Aborting.");
                            tui::press_enter_to_continue();

                            manage_user_profiles(user);
                        }
                    }

                    if *profile_string == user.username {
                        UserProfile::delete(&user.username);

                        page_header("Developer Mode - Profile Management", None);
                        println!("\nCurrent profile successfully deleted. Logging out.");
                        tui::press_enter_to_continue();

                        crate::menus::accounts::main::menu();
                    }

                    UserProfile::delete(profile_string);

                    page_header("Developer Mode - Profile Management", None);
                    println!("\nProfile '{}' successfully deleted.", profile_string);
                    tui::press_enter_to_continue();

                    manage_user_profiles(user);
                }
                None => panic!("Dialoguer picked vec index out of bounds"),
            }
        }

        2 => main(user),

        _ => panic!("Dialoguer picked option out of bounds"),
    }
}

// TODO: Bank Manipulations
fn manipulate_banks(user: &mut UserProfile) {}

// TODO: XP Manipulation
fn manipulate_xp(user: &mut UserProfile) {}

// TODO: Inventory Manipulation
fn manipulate_inventory(user: &mut UserProfile) {}

pub fn main(user: &mut UserProfile) {
    page_header(
        "Developer Settings",
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    let choice = tui::dialogue::selector(
        &[
            "1. Throw a panic",
            "2. Manipulate Inventory",
            "3. Manipulate XP",
            "4. Manipulate Banks",
            "5. Manage User Profiles",
            "NAV: Go Back",
        ],
        0,
        Some(""),
    );

    match choice {
        0 => panic!("This is a panic!"),
        1 => manipulate_inventory(user),
        2 => manipulate_xp(user),
        3 => manipulate_banks(user),
        4 => manage_user_profiles(user),
        5 => crate::menus::game::main::menu(user),
        _ => panic!("Dialogue picked option out of bounds"),
    }
}
