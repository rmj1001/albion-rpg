use albion_termrpg::lib::{
    input::prompt_input,
    tui::{self, page_header},
    user::profile::{ProfileRetrievalResult, UserProfile},
};

pub fn menu() {
    page_header("Registration", None);

    let username: String = prompt_input("Username");

    let found_profile = UserProfile::retrieve(&username);

    if let ProfileRetrievalResult::Some(_) = found_profile {
        println!("\nThat profile already exists.");
        tui::press_enter_to_continue();
        crate::menus::accounts::main::menu();
    }

    let password: String = prompt_input("Password");
    let confirm_pass: String = prompt_input("Confirm Password");

    if password != confirm_pass {
        println!("\nPasswords do not match.");
        tui::press_enter_to_continue();
        crate::menus::accounts::main::menu();
    }

    let profile = UserProfile::from(&username, &password);

    profile.save();
    println!("\nRegistration successful.");
    tui::press_enter_to_continue();
    crate::menus::accounts::main::menu();
}
