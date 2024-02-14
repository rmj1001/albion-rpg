use crate::{
    player::profile::Player,
    utils::tui::{page_header, press_enter_to_continue, HeaderSubtext},
};

pub fn main(player: &mut Player) {
    page_header("The Stronghold", HeaderSubtext::None);
    println!("You have entered the stronghold. You must win 50 battles to complete this.");
    press_enter_to_continue();

    let mut battles: usize = 0;

    while battles < 50 {
        crate::combat::battle::battle("The Stronghold", "You delve into the Stronghold...", player, true);
        battles += 1;
    }

    page_header("The Stronghold", HeaderSubtext::None);
    println!("You have successfully completed the stronghold and won the game! Congratulations!");
    player.achievements.stronghold_defeated = true;
    player.save();

    press_enter_to_continue();
    crate::menus::game_menu::main(player);
}
