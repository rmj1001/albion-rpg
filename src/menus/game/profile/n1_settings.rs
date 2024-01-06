use anglandia_text_rpg::lib::{
    terminal::{self, clear_screen},
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn main(user: &mut UserProfile) {
    loop {
        page_header("Profile Settings");
        println!("Type the menu code (ex. 1) and press ENTER/RETURN");

        println!("1. Lock Profile");
        println!("2. Delete Profile");

        let choice = tui::dialogue::prompt_input("Enter Menu Code").to_lowercase();

        match &choice[..] {
            "1" => {
                user.locked = true;
                user.save_profile();
                terminal::exit();
            }

            "2" => {
                UserProfile::delete_profile(&user.username);
                clear_screen();
                println!("Profile sucessfully deleted.");
                tui::press_enter_to_continue();
                crate::menus::accounts::main::menu();
            }

            // Enable/Disable developer mode
            "3141592" => {
                let message;
                if user.developer {
                    user.developer = false;
                    message = "Developer mode disabled.";
                } else {
                    user.developer = true;
                    message = "Developer mode enabled.";
                }

                clear_screen();
                println!("{}", message);
                tui::press_enter_to_continue();
                crate::menus::game::main::menu(user);
            }

            _ => {}
        }
    }
}
