use crate::{
    player::settings::Settings,
    utils::{
        crypt,
        input::*,
        messages::*,
        tui::{self, page_header},
    },
};

use crate::player::profile::Player;

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
    let profile_result = Player::retrieve(&username);

    match profile_result {
        Ok(profile) => {
            let mut profile = profile;

            if profile.settings.locked {
                let unlock_profile: bool = confirm("\nProfile is locked. Unlock?");

                if unlock_profile {
                    if get_password(&profile) {
                        Settings::unlock(&mut profile);
                        success_msg("Profile unlocked.");
                    } else {
                        profile_remains_locked()
                    }
                } else {
                    cancelling();
                    crate::menus::accounts::main();
                }
            }

            if !get_password(&profile) {
                crate::menus::accounts::main();
            }

            success();

            crate::menus::game_menu::main(&mut profile);
        }

        Err(message) => {
            failure(message);
            crate::menus::accounts::main();
        }
    }
}
