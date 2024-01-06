use anglandia_text_rpg::lib::{
    terminal,
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn menu(user: UserProfile) {
    loop {
        page_header(&format!("Game Menu (user: {})", user.username));
        println!(
            "Type the menu item code (ex. c3) and press ENTER/RETURN to perform that action.\n"
        );

        // TODO: Implement menu logic
        tui::sub_header("Combat");
        println!("c1. Wander the Realm");
        println!("c2. Enter the Stronghold");
        println!("\n");

        tui::sub_header("Economy");
        println!("e1. Work in the Guilds");
        println!("e2. The Bank");
        println!("e3. Trading Post");
        println!("e4. Weapons Shop");
        println!("e5. Armor Shop");
        println!("e6. Mystic Shop");
        println!("e7. Max's Shop");

        tui::sub_header("Profile");
        println!("p1. Inventory");
        println!("p2. Hall of Records");

        println!("\n");
        println!("n1. Logout");
        println!("n2. Exit Game");

        // TODO: remove this once menu logic is completed.
        tui::press_enter_to_continue();
        terminal::exit();
    }
}
