use std::process::Command;

use crate::utils::messages::success_msg;

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
pub fn exit() {
    page_header("Thanks!", super::tui::HeaderSubtext::None);
    success_msg("Thanks for playing!");

    clearscr();
    std::process::exit(0);
}
