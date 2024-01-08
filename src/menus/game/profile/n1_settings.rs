use albion_termrpg::lib::{
    input::{prompt_input, selector},
    tui::{self, page_header, HeaderInstructions},
    user::profile::UserProfile,
};

fn change_username(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderInstructions::None);
    let new_username = prompt_input("New Username");

    if new_username == user.username {
        println!("\nThis is your current username. Aborting.");
        tui::press_enter_to_continue();
        menu(user);
    }

    let confirm_username = prompt_input("Confirm Username");

    if new_username != confirm_username {
        println!("\nUsernames do not match.");
        tui::press_enter_to_continue();
        menu(user);
    }

    user.change_username(new_username);

    page_header("Profile Settings", HeaderInstructions::None);
    println!("Successfully changed username.");
    tui::press_enter_to_continue();

    menu(user);
}

fn change_password(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderInstructions::None);
    let new_password = prompt_input("New Password");

    if new_password == user.password {
        println!("\nThis is your current password Aborting.");
        tui::press_enter_to_continue();
        menu(user);
    }

    let confirm_password = prompt_input("Confirm Password");

    if new_password != confirm_password {
        println!("\nPasswords do not match.");
        tui::press_enter_to_continue();
        menu(user);
    }

    user.change_password(new_password);

    page_header("Profile Settings", HeaderInstructions::None);
    println!("Successfully changed password.");
    tui::press_enter_to_continue();

    menu(user);
}

fn lock_profile(user: &mut UserProfile) {
    user.lock();

    page_header("Profile Settings", HeaderInstructions::None);
    println!("Profile sucessfully locked.");
    tui::press_enter_to_continue();

    crate::menus::accounts::main::menu();
}

fn delete_profile(user: &mut UserProfile) {
    user.delete();

    page_header("Profile Settings", HeaderInstructions::None);
    println!("Profile sucessfully deleted.");
    tui::press_enter_to_continue();

    crate::menus::accounts::main::menu();
}

fn disable_developer_mode(user: &mut UserProfile) {
    let confirm =
        prompt_input("Are you sure you want to disable developer mode? (y/n)").to_lowercase();

    match &confirm[..] {
        "n" => {
            println!("\nAborting.");
            tui::press_enter_to_continue();
            menu(user);
        }
        "no" => {
            println!("\nAborting.");
            tui::press_enter_to_continue();
            menu(user);
        }
        "y" => {}
        "yes" => {}
        _ => {
            tui::invalid_input(None);
            menu(user);
        }
    }

    user.set_developer(false);
    println!("\nDeveloper mode disabled.");
    tui::press_enter_to_continue();

    menu(user);
}

pub fn menu(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderInstructions::Keyboard);

    #[allow(clippy::needless_late_init)]
    let choice: usize;

    if user.is_developer {
        choice = selector(
            &[
                "1. Change Username",
                "2. Change Password",
                "3. Lock Profile",
                "4. Delete Profile",
                "5. Disable Developer Mode",
                "NAV: Go Back",
            ],
            0,
            None,
        );
    } else {
        choice = selector(
            &[
                "1. Change Username",
                "2. Change Password",
                "3. Lock Profile",
                "4. Delete Profile",
                "NAV: Go Back",
            ],
            0,
            None,
        );
    }

    match choice {
        0 => change_username(user),
        1 => change_password(user),
        2 => lock_profile(user),
        3 => delete_profile(user),
        4 => {
            if user.is_developer {
                disable_developer_mode(user);
            } else {
                crate::menus::game::main::menu(user)
            }
        }
        5 => {
            if user.is_developer {
                crate::menus::game::main::menu(user)
            } else {
                panic!("Dialoguer chose index out of bounds");
            }
        }
        _ => panic!("Dialoguer chose index out of bounds"),
    }
}
