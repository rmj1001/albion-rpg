use anglandia_text_rpg::lib::{
    terminal,
    tui::{self, page_header},
    user_profile::UserProfile,
};

pub fn menu(user: UserProfile) {
    loop {
        page_header(&format!("Game Menu (user: {})", user.username));
        tui::press_enter_to_continue();

        // TODO: Implement menu logic
        terminal::exit();
    }
}
