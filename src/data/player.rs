/*!
# Player Data

Provides all the data for the player and passed around all menu functions.

# Data Points

- Health
- XP/Levels
- Achievements
- Finances
- Guilds/Jobs
- Equipment, Armor, and Weapons
- Shop items and medicine
- Settings

*/
use std::{fmt::Display, path::Path};

use crate::{
    data::{
        achievements::Achievements,
        guilds::Guilds,
        health::Health,
        inventory::{armor, bank::Bank, equipment::Equipment, items, weapons::Inventory},
        settings::Settings,
        xp::XP,
    },
    panic_menu,
    prelude::{
        confirm, delete_file, error, page_header, pause, player_file_path, read_file, warning, write_file,
        Instructions, Logs,
    },
};
use serde::{Deserialize, Serialize};
use toml as encoder;

/**
# Player Data

Provides all the data for the player and passed around all menu functions.

# Data Points

- Health
- XP/Levels
- Achievements
- Finances
- Guilds/Jobs
- Equipment, Armor, and Weapons
- Shop items and medicine
- Settings

# Usage

```
use albion_terminal_rpg::data::player::Player;
use albion_terminal_rpg::prelude::generate_hash;

let mut default_player = Player::default();

let username = "HelloThere";
let password = "testing";
let hash = generate_hash(&password);

let mut player = Player::new(&username, &hash, true);
let player_string = default_player.to_string();
```
*/
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Player {
    pub health: Health,
    pub xp: XP,
    pub achievements: Achievements,
    pub bank: Bank,
    pub guilds: Guilds,
    pub equipment: Equipment,
    pub items: items::Inventory,
    pub armor: armor::Inventory,
    pub weapons: Inventory,
    pub settings: Settings,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = encoder::to_string_pretty(&self);

        match string {
            Ok(string) => write!(f, "{string}"),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

impl TryFrom<String> for Player {
    type Error = error::Profile;
    fn try_from(data: String) -> std::result::Result<Self, Self::Error> {
        let user_result = encoder::from_str(&data);

        match user_result {
            Ok(profile) => Ok(profile),
            Err(_) => Err(error::Profile::Corrupted),
        }
    }
}

impl Player {
    /**
    Create a new player instance with a username and password.

    # Example

    ```
    use albion_terminal_rpg::prelude::generate_hash;
    use albion_terminal_rpg::data::player::Player;

    let username = "HelloThere";
    let password = "testing";
    let hash = generate_hash(&password);

    let mut new_player = Player::new(&username, &hash, true);
    ```
    */
    pub fn new<T: Display, U: Display>(username: &T, password_hash: &U, save: bool) -> Self {
        let profile: Player = Player {
            settings: Settings::new(username, password_hash),
            ..Default::default()
        };

        if save {
            profile.save();
        }

        profile
    }

    /**
    Reset all player settings, except for username and password hash.

    # Example

    ```
    use albion_terminal_rpg::data::player::Player;

    let mut player = Player::default();

    player.reset();
    ```
    */
    pub fn reset(&mut self) {
        let new_profile = Self::new(&self.settings.username, &self.settings.password_hash, true);
        *self = new_profile;
    }

    /**
    Reset all progress and inventory (not settings) without deleting the profile

    # Example

    ```
    use albion_terminal_rpg::data::player::Player;

    let mut player = Player::default();

    player.die();
    ```
    */
    pub fn die(&mut self) {
        // Wealth
        self.bank.wallet = 0;

        // Equipment
        self.equipment.reset();
        self.armor.reset();
        self.weapons.reset();
        self.items.reset();

        // Statistics
        self.xp.reset();
        self.achievements.reset();
        self.health.reset();

        self.save();
    }

    /**
    Save player data to disk.

    # Example

    ```
    use albion_terminal_rpg::data::player::Player;

    let player = Player::default();

    player.save();
    ```
    */
    pub fn save(&self) {
        let serialize_result = self.to_string();

        let path = player_file_path(&self.settings.username);
        write_file(&path, serialize_result);
    }

    /**
    Delete the **current** player from the disk.

    # Example

    ```
    use albion_terminal_rpg::data::player::Player;
    use albion_terminal_rpg::prelude::{success, failure};

    let mut player = Player::default();

    match player.delete() {
        Ok(()) => success(None),
        Err(error) => failure(&error.to_string()),
    }
    ```
    */
    pub fn delete(&mut self) -> error::Result<()> {
        Player::delete_from(&self.settings.username)
    }

    /// Delete the player file on disk
    pub fn delete_from<T: Display>(username: &T) -> error::Result<()> {
        let profile_path = player_file_path(username);
        let exists = Path::new(&profile_path).exists();

        if exists {
            delete_file(&profile_path);
            Ok(())
        } else {
            Err(Box::new(error::Profile::DoesNotExist))
        }
    }

    /**
    Get a Player from the disk.

    # Example

    ```
    use albion_terminal_rpg::data::player::Player;
    use albion_terminal_rpg::prelude::{success, failure};

    let username = "HelloThere";
    let mut player_result = Player::get(&username);

    match player_result {
        Ok(player) => success(Some(&format!("Found player '{}'.", player.settings.username))),
        Err(error) => failure(&error.to_string())
    }
    ```
    */
    pub fn get<T: Display>(username: &T) -> error::Result<Player> {
        let profile_path: String = player_file_path(username);

        let file_result: error::Result<String> = read_file(&profile_path);

        let Ok(contents) = file_result else {
            return Err(Box::new(error::Profile::DoesNotExist));
        };

        if let Ok(player) = Self::try_from(contents) {
            Ok(player)
        } else {
            println!("{}", Logs::Failure.paint("\nPlayer data file is corrupted.\n"));

            let delete: bool = confirm("Delete file?");

            if !delete {
                warning(Some("Cancelling."));
                return Err(Box::new(error::Data::Decode));
            }

            warning(Some("Deleting player data file."));

            if let Err(message) = Player::delete_from(username) {
                panic_menu!(message);
            }

            Err(Box::new(error::Data::Decode))
        }
    }

    /**
    View player data in pages (in-game)

    # Example

    ```
    use albion_terminal_rpg::data::player::Player;

    let mut player = Player::default();

    Player::paginate(&player);
    ```
    */
    pub fn paginate(player: &Player) {
        let player_string = player.to_string();
        let pages = player_string.split("\n\n");
        let total_pages = pages.clone().count();
        let mut page_number: usize = 1;

        pages.for_each(|page| {
            page_header(
                format!(
                    "Player Profile - {} - Page {}/{}",
                    player.settings.username, page_number, total_pages
                ),
                &Instructions::None,
            );

            println!("{page}\n");
            pause();

            page_number += 1;
        });
    }

    /**
    View the **CURRENT** player's data in pages (in-game)

    # Example

    ```
    use albion_terminal_rpg::data::player::Player;

    let mut player = Player::default();

    player.view();
    ```
    */
    pub fn view(&self) {
        Self::paginate(self);
    }
}
