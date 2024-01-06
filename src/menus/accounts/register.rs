use anglandia_text_rpg::lib::{
    tui::{self, dialogue, page_header},
    user_profile::UserProfile,
};

pub fn menu() {
    page_header("Registration", None);

    let username: String = dialogue::prompt_input("Username");

    let found_profile = UserProfile::retrieve_profile(&username);

    if found_profile.is_some() {
        println!("That profile already exists.");
        tui::press_enter_to_continue();
        crate::menus::accounts::main::menu();
    }

    let password: String = dialogue::prompt_input("Password");
    let confirm_pass: String = dialogue::prompt_input("Confirm Password");

    if password != confirm_pass {
        println!("\nPasswords do not match.");
        tui::press_enter_to_continue();
        crate::menus::accounts::main::menu();
    }

    let profile = UserProfile {
        username,
        password,
        ..Default::default()
    };

    profile.save_profile();
    println!("\nRegistration successful.");
    tui::press_enter_to_continue();
    crate::menus::accounts::main::menu();
}
