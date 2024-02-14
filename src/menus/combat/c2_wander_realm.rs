use crate::{
    player::profile::UserProfile,
    utils::tui::{self, page_header, HeaderSubtext},
};

pub fn main(player: &mut UserProfile) {
    page_header("Wandering the Realm", HeaderSubtext::None);

    println!("You are wandering the realm...");
    tui::sleep(3);

    crate::combat::battle::battle(player, false);
}
