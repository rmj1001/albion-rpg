use crate::{data::settings::Settings, prelude::*};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Profile Settings", Instructions::Keyboard);

    let choice: usize = select_from_str_array(
        &[
            "1. Change Username",
            "2. Change Password",
            "3. Reset Profile",
            "4. Delete Profile",
            "5. Toggle Hard Mode",
            "6. View Player Data",
            "NAV: Go Back",
        ],
        None,
    );

    match choice {
        0 => change_username(player),
        1 => change_password(player),
        2 => reset(player),
        3 => delete_profile(player),
        4 => hardmode(player),
        5 => {
            player.view();
            main(player)
        }
        6 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn change_username(player: &mut Player) {
    page_header("Profile Settings", Instructions::None);

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

    page_header("Profile Settings", Instructions::None);
    success(Some("Username changed."));

    main(player);
}

fn change_password(player: &mut Player) {
    page_header("Profile Settings", Instructions::Other("Enter new password."));

    let new_password = password(false);
    let new_pass_is_old_pass = verify_hash(&new_password, &player.settings.password_hash);

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

    success(Some("Password changed."));

    main(player);
}

fn reset(player: &mut Player) {
    page_header("Profile Settings", Instructions::None);

    let confirm_reset = confirm("Are you sure you want to reset your profile?");

    if !confirm_reset {
        cancel(None);
        main(player);
    }

    success(Some("Profile reset."));

    player.reset();
    player.save();

    main(player);
}

fn delete_profile(player: &mut Player) {
    page_header("Profile Settings", Instructions::None);

    let confirm_delete = confirm("Are you sure you want to delete your profile?");

    if !confirm_delete {
        cancel(None);
        main(player);
    }

    match player.delete() {
        Ok(_) => success(Some("Profile deleted.")),
        Err(error) => failure(&error.to_string()),
    }

    crate::menus::accounts::main();
}

fn hardmode(player: &mut Player) {
    page_header("Profile Settings", Instructions::None);

    if !player.settings.hardmode {
        println!("Are you sure you want to enable hardmode?");
        let confirmation = confirm("If you lose a battle, you could have your profile deleted.");

        if !confirmation {
            cancel(None);
            main(player);
        }
    } else {
        let confirmation = confirm("Are you sure you want to disable hardmode?");

        if !confirmation {
            cancel(None);
            main(player);
        }
    }

    Settings::toggle_hardmode(player);

    main(player);
}
