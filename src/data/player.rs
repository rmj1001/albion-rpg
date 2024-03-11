use crate::{
    data::{
        achievements::*,
        guilds::Guilds,
        health::Health,
        inventory::{armor::*, bank::*, equipment::Equipment, items::*, weapons::*},
        settings::Settings,
        xp::*,
    },
    panic_screen,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use toml as encoder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub health: Health,
    pub xp: XP,
    pub achievements: Achievements,
    pub bank: Bank,
    pub guilds: Guilds,
    pub equipment: Equipment,
    pub items: ItemInventory,
    pub armor: ArmorInventory,
    pub weapons: WeaponsInventory,
    pub settings: Settings,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            xp: XP::new(),
            health: Health::new(),
            achievements: Achievements::new(),
            bank: Bank::new(),
            guilds: Guilds::new(),
            equipment: Equipment::new(),
            items: ItemInventory::new(),
            armor: ArmorInventory::new(),
            weapons: WeaponsInventory::new(),
        }
    }
}

impl Player {
    /// Creates new instance with empty username/password fields
    pub fn new(username: &str, password_hash: &str, save: bool) -> Self {
        let profile: Player = Player {
            settings: Settings::new(username, password_hash),
            ..Default::default()
        };

        if save {
            profile.save();
        }

        profile
    }

    /// Reset all player settings, except for username and password hash
    pub fn reset(&mut self) {
        let new_profile = Self::new(&self.settings.username, &self.settings.password_hash, true);
        *self = new_profile;
    }

    /// Reset all progress and inventory (not settings) without deleting the profile
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

    /// Save player data to disk.
    pub fn save(&self) {
        let serialize_result = self.to_string();

        let path = file_handler::generate_profile_path(&self.settings.username);
        file_handler::write_file(path, serialize_result)
    }

    /// Delete the player file on disk
    pub fn delete(&mut self) {
        Player::delete_from_username(&self.settings.username);
    }

    /// Delete the player file on disk
    pub fn delete_from_username(username: &str) {
        let profile_path = file_handler::generate_profile_path(username);

        file_handler::delete_file(profile_path);
    }

    /// Retrieve player data from disk using the username as the search string
    pub fn get_from_username(username: &str) -> Result<Player> {
        let profile_path: String = file_handler::generate_profile_path(username);
        let mut contents: String = String::new();

        let file_result: Result<String> = file_handler::read_file(profile_path);

        match file_result {
            Ok(data) => contents = data,
            Err(_) => return Err(ProfileError::DoesNotExist.boxed()),
        }

        match Self::from_string(contents) {
            Ok(player) => Ok(player),
            Err(_) => {
                let delete: bool = confirm("Player data file is corrupted. Delete?");

                if delete {
                    warning(Some("Deleting player data file."));
                    Player::delete_from_username(username)
                } else {
                    warning(Some("Cancelling."));
                }

                Err(DataError::Decode.boxed())
            }
        }
    }
}

impl ToString for Player {
    fn to_string(&self) -> String {
        let string = encoder::to_string_pretty(&self);

        match string {
            Ok(string) => string,
            Err(message) => panic_screen!(message),
        }
    }
}

impl Player {
    pub fn from_string(data: String) -> Result<Player> {
        let user_result = encoder::from_str(&data);

        match user_result {
            Ok(profile) => Ok(profile),
            Err(_) => Err(ProfileError::Corrupted.boxed()),
        }
    }

    pub fn paged_viewer(player: &Player) {
        let string = player.to_string();

        let pages = string.split("\n\n");
        let total_pages = pages.clone().count();
        let mut page_number: usize = 1;

        pages.for_each(|page| {
            page_header(
                format!(
                    "Player Profile - {} - Page {}/{}",
                    player.settings.username, page_number, total_pages
                ),
                Instructions::None,
            );

            println!("{}\n", page);
            pause();

            page_number += 1;
        });
    }

    pub fn paged_view(&self) {
        Self::paged_viewer(self);
    }
}
