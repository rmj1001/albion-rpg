use anglandia_text_rpg::lib::{
    tui::dialogue,
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn menu() {
    page_header("Login");

    let username: String = dialogue::prompt_input("Username");
    let profile_result = UserProfile::retrieve_profile(&username);

    match profile_result {
        Some(profile) => {
            let mut profile = profile;
            let password: String = dialogue::password();

            if password != profile.password {
                println!("Incorrect password.");
                tui::press_enter_to_continue();
                crate::menus::accounts::main::menu();
            }

            println!("Login successful.");
            tui::press_enter_to_continue();

            crate::menus::game::main::menu(&mut profile);
        }

        None => {
            println!("That profile does not exist.");
            tui::press_enter_to_continue();
            crate::menus::accounts::main::menu();
        }
    }
}
