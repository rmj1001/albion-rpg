use crate::{
    data::settings::Settings,
    utils::{
        crypt,
        input::*,
        messages::*,
        tui::{self, page_header},
    },
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

fn profile_remains_locked() {
    cancel_msg("Profile will remain locked.");
    crate::menus::accounts::main();
}

pub fn main() {
    page_header("Login", tui::HeaderSubtext::None);

    let username: String = prompt_colon("Username");
    let profile_result: crate::Result<Player> = Player::get_from_username(&username);

    match profile_result {
        Ok(player) => {
            let mut player = player;

            if player.settings.locked {
                let unlock_profile: bool = confirm("\nProfile is locked. Unlock?");

                if unlock_profile {
                    if get_password(&player) {
                        Settings::toggle_lock(&mut player);
                    } else {
                        profile_remains_locked();
                    }
                } else {
                    cancelling();
                    crate::menus::accounts::main();
                }
            }

            if !get_password(&player) {
                crate::menus::accounts::main();
            }

            success();

            crate::menus::game_menu::main(&mut player);
        }

        Err(_) => crate::menus::accounts::main(),
    }
}
