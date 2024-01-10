use crate::lib::{
    crypt,
    input::*,
    tui::{self, page_header, press_enter_to_continue},
};

use crate::user::profile::{ProfileRetrievalResult, UserProfile};

fn get_password(profile: &UserProfile) -> bool {
    let input_password: String = password();
    let verified_password =
        crypt::verify(input_password.clone(), profile.settings.password.clone());

    if !verified_password {
        println!("\nIncorrect password.");
        return false;
    }

    true
}

fn profile_remains_locked() {
    println!("\nProfile will remain locked.");
    tui::press_enter_to_continue();
    crate::menus::accounts::main::menu();
}

pub fn menu() {
    page_header("Login", tui::HeaderInstructions::None);

    let username: String = prompt_input("Username");
    let profile_result = UserProfile::retrieve(&username);

    match profile_result {
        ProfileRetrievalResult::Some(profile) => {
            let mut profile = profile;

            if profile.settings.locked {
                let answer_option: Option<bool> = yes_or_no("\nProfile is locked. Unlock?");

                match answer_option {
                    Some(is_yes) => {
                        if is_yes {
                            if get_password(&profile) {
                                profile.settings.unlock(None);
                                println!("\nProfile unlocked. Proceed with login.\n");
                            } else {
                                profile_remains_locked()
                            }
                        } else {
                            println!("\nCancelling.");
                            press_enter_to_continue();
                            crate::menus::accounts::main::menu();
                        }
                    }

                    None => crate::menus::accounts::main::menu(),
                }
            }

            if !get_password(&profile) {
                tui::press_enter_to_continue();
                crate::menus::accounts::main::menu();
            }

            println!("\nLogin successful.");
            tui::press_enter_to_continue();

            crate::menus::game::main::menu(&mut profile);
        }

        ProfileRetrievalResult::None(message) => {
            println!("\n{}", message);
            tui::press_enter_to_continue();
            crate::menus::accounts::main::menu();
        }
    }
}
