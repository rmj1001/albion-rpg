use std::process::Command;

use crate::{
    data::player::Player,
    prelude::{sleep, success_msg},
};

use super::tui::page_header;

/// Clears the terminal screen
pub fn clearscr() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

/// Sends an exit code of 0 (no errors)
pub fn exit(player: Option<&mut Player>) {
    page_header("Thanks!", super::tui::HeaderSubtext::None);

    if let Some(player) = player {
        println!("Saving game...");
        player.save();
        sleep(2);

        success_msg("Game saved! Thanks for playing!");

        clearscr();
        std::process::exit(0);
    }

    success_msg("Thanks for playing!");

    clearscr();
    std::process::exit(0);
}
