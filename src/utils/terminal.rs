/*!
# Terminal Functions

These functions are provided for manipulating the terminal process.
*/

use crate::{
    data::player::Player,
    prelude::{page_header, success},
};

use std::{process, thread};

/**
Clears the terminal screen.

# Usage

```
use albion_terminal_rpg::prelude::clearscr;

clearscr();
```
*/
pub fn clearscr() {
    if cfg!(target_os = "windows") {
        process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        process::Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

///
/**
Custom exit screen.

**Details:**
- Save Player data if player object passed.
- Returns exit code 0 to terminal
- Clears screen before exiting.

# Examples

```
use albion_terminal_rpg::prelude::exit;
use albion_terminal_rpg::data::player::Player;

let mut example_player = Player::default();

exit(None);
exit(Some(&mut example_player));
```
 */
pub fn exit(player: Option<&mut Player>) {
    page_header("Thanks!", super::tui::Instructions::None);

    if let Some(player) = player {
        println!("Saving game...");
        player.save();
        sleep(2);

        success(Some("Game saved! Thanks for playing!"));

        clearscr();
        std::process::exit(0);
    }

    success(Some("Thanks for playing!"));

    clearscr();
    process::exit(0);
}

/**
Pause terminal output for a number of seconds.

# Usage

```
use albion_terminal_rpg::prelude::sleep;

sleep(1);
```
*/
pub fn sleep(seconds: u64) {
    thread::sleep(std::time::Duration::from_secs(seconds))
}
