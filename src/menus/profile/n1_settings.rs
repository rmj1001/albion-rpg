use crate::{
    player::settings::Settings,
    utils::{
        crypt,
        input::{confirm, password, prompt_colon, select_from_str_array},
        messages::*,
        tui::{page_header, HeaderSubtext},
    },
};

use crate::player::profile::UserProfile;

pub fn main(player: &mut UserProfile) {
    page_header("Profile Settings", HeaderSubtext::Keyboard);

    let choice: usize = select_from_str_array(
        &[
            "1. Change Username",
            "2. Change Password",
            "3. Lock Profile",
            "4. Delete Profile",
            "5. Enable Hardmode",
            "NAV: Go Back",
        ],
        None,
    );

    match choice {
        0 => change_username(player),
        1 => change_password(player),
        2 => lock_profile(player),
        3 => delete_profile(player),
        4 => hardmode(player),
        5 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn change_username(player: &mut UserProfile) {
    page_header("Profile Settings", HeaderSubtext::None);
    let new_username = prompt_colon("New Username");

    if new_username == player.settings.username {
        failure("This is your current username.");
        main(player);
    }

    let confirm_username = prompt_colon("Confirm Username");

    if new_username != confirm_username {
        failure("Usernames do not match");
        main(player);
    }

    Settings::change_username(player, new_username);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Username changed.");

    main(player);
}

fn change_password(player: &mut UserProfile) {
    page_header("Profile Settings", HeaderSubtext::Other("Enter new password."));
    let new_password = password(false);
    let new_pass_is_old_pass = crypt::verify_hash(new_password.clone(), player.settings.password.clone());

    if new_pass_is_old_pass {
        failure("This is your current password.");
        main(player);
    }

    let confirm_password = password(true);

    if new_password != confirm_password {
        failure("Passwords do not match.");
        main(player);
    }

    Settings::change_password(player, new_password);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Password changed.");

    main(player);
}

fn lock_profile(player: &mut UserProfile) {
    let confirm_lock = confirm("Are you sure you want to lock your profile?");

    if !confirm_lock {
        cancelling();
        main(player);
    }

    Settings::lock(player);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Profile locked.");

    crate::menus::accounts::main();
}

fn delete_profile(player: &mut UserProfile) {
    let confirm_delete = confirm("Are you sure you want to delete your profile?");

    if !confirm_delete {
        cancelling();
        main(player);
    }

    player.delete();

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Profile deleted.");

    crate::menus::accounts::main();
}

fn hardmode(player: &mut UserProfile) {
    if !player.settings.hardmode {
        println!("Are you sure you want to enable hardmode?");
        let confirmation = confirm("If you lose a battle, you could have your profile deleted.");

        if !confirmation {
            cancelling();
            main(player);
        }

        player.settings.hardmode = true;
    } else {
        let confirmation = confirm("Are you sure you want to disable hardmode?");

        if !confirmation {
            cancelling();
            main(player);
        }

        player.settings.hardmode = false;
    }

    main(player);
}
