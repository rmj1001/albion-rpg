use super::achievements::*;
use super::guilds::Memberships;
use super::health::Health;
use super::settings::Settings;
use super::xp::*;

use crate::data::inventory::{armor::*, bank::*, equipment::Equipment, items::*, weapons::*};

use crate::utils::files;
use crate::{DataError, ProfileError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub health: Health,
    pub xp: XP,
    pub achievements: Achievements,
    pub bank: Bank,
    pub guilds: Memberships,
    pub equipment: Equipment,
    pub items: MundaneInventory,
    pub armor: ArmorInventory,
    pub weapons: WeaponsInventory,
    pub settings: Settings,
}

impl Player {
    /// Creates new instance with empty username/password fields
    pub fn new(username: &str, password_hash: &str, save: bool) -> Self {
        let profile: Player = Player {
            settings: Settings::new(username, password_hash),
            health: Health::new(),
            xp: XP::new(),
            achievements: Achievements::new(),
            bank: Bank::new(),
            guilds: Memberships::new(),
            equipment: Equipment::new(),
            items: MundaneInventory::new(),
            armor: ArmorInventory::new(),
            weapons: WeaponsInventory::new(),
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
        let serialize_result =
            crate::utils::files::encoding::encode(self).expect("Could not convert Player to config file format.");

        let path = files::handler::generate_profile_path(&self.settings.username);
        files::handler::write_file(path, serialize_result)
    }

    /// Delete the player file on disk
    pub fn delete(&mut self) {
        Player::delete_from_username(&self.settings.username);
        self.reset();
    }

    /// Delete the player file on disk
    pub fn delete_from_username(username: &str) {
        let profile_path = files::handler::generate_profile_path(username);

        files::handler::delete_file(profile_path);
    }

    /// Retrieve player data from disk using the username as the search string
    pub fn get_from_username(username: &str) -> crate::Result<Player> {
        let profile_path = files::handler::generate_profile_path(username);
        let mut contents: String = String::new();

        let file_result = files::handler::read_file(profile_path);

        match file_result {
            Ok(data) => contents = data,
            Err(_) => return Err(ProfileError::DoesNotExist.boxed()),
        }

        match crate::utils::files::encoding::decode(contents) {
            Ok(player) => Ok(player),
            Err(_) => {
                Player::delete_from_username(username);
                Err(DataError::Decode.boxed())
            }
        }
    }
}
