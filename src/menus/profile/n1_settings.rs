use crate::lib::{
    crypt,
    input::{confirm, out_of_bounds, password, prompt_input, selector},
    tui::{self, page_header, HeaderSubtext},
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderSubtext::Keyboard);

    let choice: usize = selector(
        &[
            "1. Change Username",
            "2. Change Password",
            "3. Lock Profile",
            "4. Delete Profile",
            "NAV: Go Back",
        ],
        None,
    );

    match choice {
        0 => change_username(user),
        1 => change_password(user),
        2 => lock_profile(user),
        3 => delete_profile(user),
        4 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(None),
    }
}

fn change_username(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderSubtext::None);
    let new_username = prompt_input("New Username:");

    if new_username == user.settings.username {
        println!("\nThis is your current username.");
        tui::press_enter_to_continue();
        main(user);
    }

    let confirm_username = prompt_input("Confirm Username:");

    if new_username != confirm_username {
        println!("\nUsernames do not match.");
        tui::press_enter_to_continue();
        main(user);
    }

    user.settings.change_username(None, new_username);

    page_header("Profile Settings", HeaderSubtext::None);
    println!("Successfully changed username.");
    tui::press_enter_to_continue();

    main(user);
}

fn change_password(user: &mut UserProfile) {
    page_header(
        "Profile Settings",
        HeaderSubtext::Other("Enter new password."),
    );
    let new_password = password(false);
    let new_pass_is_old_pass =
        crypt::verify_hash(new_password.clone(), user.settings.password.clone());

    if new_pass_is_old_pass {
        println!("\nThis is your current password.");
        tui::press_enter_to_continue();
        main(user);
    }

    let confirm_password = password(true);

    if new_password != confirm_password {
        println!("\nPasswords do not match.");
        tui::press_enter_to_continue();
        main(user);
    }

    user.settings.change_password(None, new_password);

    page_header("Profile Settings", HeaderSubtext::None);
    println!("Successfully changed password.");
    tui::press_enter_to_continue();

    main(user);
}

fn lock_profile(user: &mut UserProfile) {
    let confirm_lock = confirm("Are you sure you want to lock your profile?");

    if !confirm_lock {
        println!("\nCancelling.");
        tui::press_enter_to_continue();
        main(user);
    }

    user.settings.lock(None);

    page_header("Profile Settings", HeaderSubtext::None);
    println!("Profile sucessfully locked.");
    tui::press_enter_to_continue();

    crate::menus::accounts::main();
}

fn delete_profile(user: &mut UserProfile) {
    let confirm_delete = confirm("Are you sure you want to delete your profile?");

    if !confirm_delete {
        println!("\nCancelling.");
        tui::press_enter_to_continue();
        main(user);
    }

    user.delete();

    page_header("Profile Settings", HeaderSubtext::None);
    println!("Profile sucessfully deleted.");
    tui::press_enter_to_continue();

    crate::menus::accounts::main();
}
