use super::achievements::*;
use super::guilds::GuildMemberships;
use super::health::Health;
use super::settings::Settings;
use super::xp::*;

use crate::player::inventory::{armor::*, bank::*, equipment::Equipment, items::*, weapons::*};

use crate::utils::files;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub health: Health,
    pub xp: XP,
    pub achievements: Achievements,
    pub bank: Bank,
    pub guild_memberships: GuildMemberships,
    pub equipment: Equipment,
    pub items: MundaneInventory,
    pub armor: ArmorInventory,
    pub weapons: WeaponsInventory,
    pub settings: Settings,
}

impl Player {
    /// Creates new instance with empty username/password fields
    pub fn new(username: &str, password_hash: &str) -> Self {
        let profile: Player = Player {
            settings: Settings::new(username, password_hash),
            health: Health::new(),
            xp: XP::new(),
            achievements: Achievements::new(),
            bank: Bank::new(),
            guild_memberships: GuildMemberships::new(),
            equipment: Equipment::new(),
            items: MundaneInventory::new(),
            armor: ArmorInventory::new(),
            weapons: WeaponsInventory::new(),
        };

        profile.save();

        profile
    }

    pub fn reset(&mut self) {
        let new_profile = Self::new(&self.settings.username, &self.settings.password_hash);
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

    pub fn save(&self) {
        let serialize_result = crate::utils::files::encoding::serialize_user(self)
            .expect("Could not convert Player to config file format.");

        let path = files::handler::generate_profile_path(&self.settings.username);
        files::handler::write_file(path, serialize_result)
    }

    pub fn delete(&self) {
        Player::delete_from_username(&self.settings.username);
    }

    pub fn delete_from_username(username: &str) {
        let profile_path = files::handler::generate_profile_path(username);

        files::handler::delete_file(profile_path);
    }

    pub fn get_from_username(username: &str) -> Result<Player, String> {
        let profile_path = files::handler::generate_profile_path(username);
        let mut contents: String = String::new();

        let file_result = files::handler::read_file(profile_path);

        match file_result {
            Ok(data) => contents = data,
            Err(_) => return Err(format!("Profile '{}' does not exist.", username)),
        }

        match crate::utils::files::encoding::deserialize_user(contents) {
            Ok(player) => Ok(player),
            Err(message) => {
                Player::delete_from_username(username);
                Err(message)
            }
        }
    }
}
