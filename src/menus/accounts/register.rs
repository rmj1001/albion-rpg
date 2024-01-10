use crate::lib::{
    crypt,
    input::prompt_input,
    tui::{self, page_header, HeaderInstructions},
};

use crate::user::profile::{ProfileRetrievalResult, UserProfile};

pub fn main() {
    page_header("Registration", HeaderInstructions::None);

    let username: String = prompt_input("Username");

    let found_profile = UserProfile::retrieve(&username);

    if let ProfileRetrievalResult::Some(_) = found_profile {
        println!("\nThat profile already exists.");
        tui::press_enter_to_continue();
        crate::menus::accounts::main::main();
    }

    let password: String = prompt_input("Password");
    let confirm_pass: String = prompt_input("Confirm Password");

    if password != confirm_pass {
        println!("\nPasswords do not match.");
        tui::press_enter_to_continue();
        crate::menus::accounts::main::main();
    }

    let password_hash = crypt::generate(password);

    let profile = UserProfile::from(&username, &password_hash);

    profile.save();
    println!("\nRegistration successful.");
    tui::press_enter_to_continue();
    crate::menus::accounts::main::main();
}
