use anglandia_text_rpg::lib::{
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn menu(user: &mut UserProfile) {
    page_header(
        "Profile Settings",
        Some("Type the menu code (ex. 1) and press ENTER/RETURN"),
    );

    println!("1. Change Username");
    println!("2. Change Password");
    println!("3. Lock Profile");
    println!("4. Delete Profile\n");

    if user.is_developer {
        println!("5. Disable Developer Mode");
    }

    println!("0. Go Back to Main Menu\n");

    let choice = tui::dialogue::prompt_input("Enter Menu Code").to_lowercase();

    match &choice[..] {
        "1" => {
            page_header("Profile Settings", None);
            let new_username = tui::dialogue::prompt_input("New Username");

            if new_username == user.username {
                println!("\nThis is your current username. Aborting.");
                tui::press_enter_to_continue();
                menu(user);
            }

            let confirm_username = tui::dialogue::prompt_input("Confirm Username");

            if new_username != confirm_username {
                println!("\nUsernames do not match.");
                tui::press_enter_to_continue();
                menu(user);
            }

            user.change_username(new_username);

            page_header("Profile Settings", None);
            println!("Successfully changed username.");
            tui::press_enter_to_continue();

            menu(user);
        }
        "2" => {
            page_header("Profile Settings", None);
            let new_password = tui::dialogue::prompt_input("New Password");

            if new_password == user.password {
                println!("\nThis is your current password Aborting.");
                tui::press_enter_to_continue();
                menu(user);
            }

            let confirm_password = tui::dialogue::prompt_input("Confirm Password");

            if new_password != confirm_password {
                println!("\nPasswords do not match.");
                tui::press_enter_to_continue();
                menu(user);
            }

            user.change_password(new_password);

            page_header("Profile Settings", None);
            println!("Successfully changed password.");
            tui::press_enter_to_continue();

            menu(user);
        }
        "3" => {
            user.lock();

            page_header("Profile Settings", None);
            println!("Profile sucessfully locked.");
            tui::press_enter_to_continue();

            crate::menus::accounts::main::menu();
        }

        "4" => {
            user.delete_profile(None);

            page_header("Profile Settings", None);
            println!("Profile sucessfully deleted.");
            tui::press_enter_to_continue();

            crate::menus::accounts::main::menu();
        }

        "5" => {
            if !user.is_developer {
                tui::invalid_input(Some("Invalid input."));
                menu(user);
            }

            user.developer_mode(false);
            println!("\nDeveloper mode disabled.");
            tui::press_enter_to_continue();

            menu(user);
        }

        "0" => crate::menus::game::main::menu(user),

        // Enable/Disable developer mode
        "3141592" => {
            #[allow(clippy::needless_late_init)]
            let message;

            if user.is_developer {
                user.developer_mode(false);
                message = "\nDeveloper mode disabled.";
            } else {
                user.developer_mode(true);
                message = "\nDeveloper mode enabled.";
            }

            println!("{}", message);
            tui::press_enter_to_continue();
            menu(user);
        }

        wrong_input => {
            tui::invalid_input(Some(wrong_input));
            menu(user);
        }
    }
}
