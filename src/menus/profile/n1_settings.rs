use crate::{
    lib::{
        crypt,
        input::{confirm, password, prompt_colon, select_from_str_array},
        messages::*,
        tui::{page_header, HeaderSubtext},
    },
    user::settings::Settings,
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderSubtext::Keyboard);

    let choice: usize = select_from_str_array(
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
        _ => out_of_bounds(),
    }
}

fn change_username(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderSubtext::None);
    let new_username = prompt_colon("New Username");

    if new_username == user.settings.username {
        failure("This is your current username.");
        main(user);
    }

    let confirm_username = prompt_colon("Confirm Username");

    if new_username != confirm_username {
        failure("Usernames do not match");
        main(user);
    }

    Settings::change_username(user, new_username);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Username changed.");

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
        failure("This is your current password.");
        main(user);
    }

    let confirm_password = password(true);

    if new_password != confirm_password {
        failure("Passwords do not match.");
        main(user);
    }

    Settings::change_password(user, new_password);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Password changed.");

    main(user);
}

fn lock_profile(user: &mut UserProfile) {
    let confirm_lock = confirm("Are you sure you want to lock your profile?");

    if !confirm_lock {
        cancelling();
        main(user);
    }

    Settings::lock(user);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Profile locked.");

    crate::menus::accounts::main();
}

fn delete_profile(user: &mut UserProfile) {
    let confirm_delete = confirm("Are you sure you want to delete your profile?");

    if !confirm_delete {
        cancelling();
        main(user);
    }

    user.delete();

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Profile deleted.");

    crate::menus::accounts::main();
}
