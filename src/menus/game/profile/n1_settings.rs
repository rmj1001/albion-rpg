use anglandia_text_rpg::lib::{
    terminal::clear_screen,
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn main(user: &mut UserProfile) {
    loop {
        page_header(
            "Profile Settings",
            Some("Type the menu code (ex. 1) and press ENTER/RETURN"),
        );

        println!("1. Lock Profile");
        println!("2. Delete Profile");
        println!("3. Go Back to Main Menu\n");

        let choice = tui::dialogue::prompt_input("Enter Menu Code").to_lowercase();

        match &choice[..] {
            "1" => {
                user.lock_profile();

                clear_screen();
                println!("Profile sucessfully locked.");
                tui::press_enter_to_continue();

                crate::menus::accounts::main::menu();
            }

            "2" => {
                user.delete_profile(None);

                clear_screen();
                println!("Profile sucessfully deleted.");
                tui::press_enter_to_continue();

                crate::menus::accounts::main::menu();
            }

            "3" => crate::menus::game::main::menu(user),

            // Enable/Disable developer mode
            "3141592" => {
                #[allow(clippy::needless_late_init)]
                let message;

                if user.developer {
                    user.developer_mode(false);
                    message = "Developer mode disabled.";
                } else {
                    user.developer_mode(true);
                    message = "Developer mode enabled.";
                }

                clear_screen();
                println!("{}", message);
                tui::press_enter_to_continue();
                crate::menus::game::main::menu(user);
            }

            wrong_input => {
                tui::invalid_input(Some(wrong_input));
                continue;
            }
        }
    }
}
