use crate::utils::{
    crypt,
    input::*,
    messages::*,
    tui::{self, page_header},
};

use crate::data::player::Player;

fn get_password(profile: &Player) -> bool {
    let input_password: String = password(false);
    let verified_password = crypt::verify_hash(input_password.clone(), profile.settings.password_hash.clone());

    if !verified_password {
        failure("Incorrect password.");
        return false;
    }

    true
}

pub fn main() {
    page_header("Login", tui::HeaderSubtext::None);

    let username: String = prompt_colon("Username");
    let profile_result: crate::Result<Player> = Player::get_from_username(&username);

    match profile_result {
        Ok(player) => {
            let mut player = player;

            if !get_password(&player) {
                crate::menus::accounts::main();
            }

            success();

            crate::menus::game_menu::main(&mut player);
        }

        Err(message) => {
            message.failure();
            crate::menus::accounts::main();
        }
    }
}
