use albion_termrpg::lib::{
    crypt,
    input::{out_of_bounds, prompt_input, selector},
    tui::{self, page_header, HeaderInstructions},
    user::profile::UserProfile,
};

fn change_username(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderInstructions::None);
    let new_username = prompt_input("New Username");

    if new_username == user.username {
        println!("\nThis is your current username.");
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
    let new_pass_is_old_pass = crypt::verify(new_password.clone(), user.password.clone());

    if new_pass_is_old_pass {
        println!("\nThis is your current password.");
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

pub fn menu(user: &mut UserProfile) {
    page_header("Profile Settings", HeaderInstructions::Keyboard);

    #[allow(clippy::needless_late_init)]
    let choice: usize = selector(
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

    match choice {
        0 => change_username(user),
        1 => change_password(user),
        2 => lock_profile(user),
        3 => delete_profile(user),
        4 => crate::menus::game::main::menu(user),
        _ => out_of_bounds(None),
    }
}
