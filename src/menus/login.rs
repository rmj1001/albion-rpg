use crate::prelude::{error, failure, page_header, password, prompt, success, verify_hash, Instructions};

use crate::data::player::Player;

fn get_password(profile: &Player) -> bool {
    let input_password: String = password(false);
    let verified_password = verify_hash(&input_password, &profile.settings.password_hash);

    if !verified_password {
        failure("Incorrect password.");
        return false;
    }

    true
}

pub fn main() {
    page_header("Login", &Instructions::None);

    let username: String = prompt("Username");
    let profile_result: error::Result<Player> = Player::get(&username);

    match profile_result {
        Ok(player) => {
            let mut player = player;

            if !get_password(&player) {
                crate::menus::accounts::main();
            }

            success(None);

            crate::menus::game_menu::main(&mut player);
        }

        Err(error) => {
            error.print(true);
            crate::menus::accounts::main();
        }
    }
}
