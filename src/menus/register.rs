use crate::{
    player::profile::UserProfile,
    utils::{
        crypt,
        input::{self, prompt_colon},
        messages::{failure, success},
        tui::{page_header, HeaderSubtext},
    },
};

pub fn main() {
    page_header("Registration", HeaderSubtext::None);

    let username: String = prompt_colon("Username");

    let found_profile = UserProfile::retrieve(&username);

    if found_profile.is_ok() {
        failure(format!("Profile '{}' already exists.", username));
        crate::menus::accounts::main();
    }

    let password: String = input::password(false);
    let confirm_pass: String = input::password(true);

    if password != confirm_pass {
        failure("Passwords do not match.");
        crate::menus::accounts::main();
    }

    let password_hash = crypt::generate_hash(password);

    let profile = UserProfile::from(&username, &password_hash);

    profile.save();
    success();
    crate::menus::accounts::main();
}
