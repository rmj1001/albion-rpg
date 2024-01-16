use crate::lib::{
    crypt,
    input::*,
    tui::{self, page_header, press_enter_to_continue},
};

use crate::user::profile::{ProfileRetrievalResult, UserProfile};

fn get_password(profile: &UserProfile) -> bool {
    let input_password: String = password(false);
    let verified_password =
        crypt::verify_hash(input_password.clone(), profile.settings.password.clone());

    if !verified_password {
        println!("\nIncorrect password.");
        return false;
    }

    true
}

fn profile_remains_locked() {
    println!("\nProfile will remain locked.");
    tui::press_enter_to_continue();
    crate::menus::accounts::main();
}

pub fn main() {
    page_header("Login", tui::HeaderSubtext::None);

    let username: String = prompt_input("Username");
    let profile_result = UserProfile::retrieve(&username);

    match profile_result {
        ProfileRetrievalResult::Some(profile) => {
            let mut profile = profile;

            if profile.settings.locked {
                let unlock_profile: bool = confirm("\nProfile is locked. Unlock?");

                if unlock_profile {
                    if get_password(&profile) {
                        profile.settings.unlock(None);
                        println!("\nProfile unlocked. Proceed with login.\n");
                    } else {
                        profile_remains_locked()
                    }
                } else {
                    println!("\nCancelling.");
                    press_enter_to_continue();
                    crate::menus::accounts::main();
                }
            }

            if !get_password(&profile) {
                tui::press_enter_to_continue();
                crate::menus::accounts::main();
            }

            println!("\nLogin successful.");
            tui::press_enter_to_continue();

            crate::menus::game_menu::main(&mut profile);
        }

        ProfileRetrievalResult::None(message) => {
            println!("\n{}", message);
            tui::press_enter_to_continue();
            crate::menus::accounts::main();
        }
    }
}
