use crate::{data::player::Player, prelude::*};

pub fn main() {
    page_header("Registration", &Instructions::None);

    let username: String = prompt("Username");

    let found_profile = Player::get(&username);

    if found_profile.is_ok() {
        failure(&format!("Profile '{username}' already exists."));
        crate::menus::accounts::main();
    }

    let user_password: String = password(false);
    let confirm_pass: String = password(true);

    if user_password != confirm_pass {
        failure("Passwords do not match.");
        crate::menus::accounts::main();
    }

    let password_hash = generate_hash(&user_password);

    let profile = Player::new(&username, &password_hash, true);

    profile.save();
    success(None);
    crate::menus::accounts::main();
}
