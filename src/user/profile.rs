use super::achievements::*;
use super::armor::*;
use super::bank::*;
use super::guilds::Guild;
use super::guilds::GuildMemberships;
use super::inventory::*;
use super::settings::Settings;
use super::weapons::*;
use super::xp::*;

use crate::misc::files;
use crate::misc::files::file_path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Health {
    pub hp: usize,
    pub hunger: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    pub health: Health,
    pub xp: XP,
    pub achievements: Achievements,
    pub bank: Bank,
    pub guild_memberships: GuildMemberships,
    pub inventory: MundaneInventory,
    pub armor: ArmorInventory,
    pub weapons: WeaponsInventory,
    pub settings: Settings,
}

impl UserProfile {
    /// Creates new instance with empty username/password fields
    pub fn new() -> Self {
        let profile: UserProfile = UserProfile {
            settings: Settings {
                username: String::new(),
                password: String::new(),
                developer: false,
                locked: false,
            },
            health: Health { hp: 100, hunger: 0 },
            xp: XP {
                combat: 0,
                fishing: 0,
                cooking: 0,
                woodcutting: 0,
                mining: 0,
                smithing: 0,
                thieving: 0,
            },
            achievements: Achievements {
                monsters_killed: 0,
                earned_million_gold: false,
                level_100_reached: false,
                stronghold_defeated: false,
                hacked_the_game: false,
            },
            bank: Bank {
                wallet: 0,
                account1: 0,
                account2: 0,
                account3: 0,
                account4: 0,
            },
            guild_memberships: GuildMemberships {
                fishing: Guild {
                    member: false,
                    member_price: 100,
                },
                cooking: Guild {
                    member: false,
                    member_price: 150,
                },
                woodcutting: Guild {
                    member: false,
                    member_price: 300,
                },
                mining: Guild {
                    member: false,
                    member_price: 500,
                },
                smithing: Guild {
                    member: false,
                    member_price: 1000,
                },
            },
            inventory: MundaneInventory {
                bait: Item {
                    name: "Bait".to_string(),
                    price: 1,
                    quantity: 0,
                },
                seeds: Item {
                    name: "Seeds".to_string(),
                    price: 1,
                    quantity: 0,
                },
                furs: Item {
                    name: "Fur".to_string(),
                    price: 5,
                    quantity: 0,
                },
                fish: Item {
                    name: "Fish".to_string(),
                    price: 10,
                    quantity: 0,
                },
                food: Item {
                    name: "Food".to_string(),
                    price: 25,
                    quantity: 0,
                },
                wood: Item {
                    name: "Wood".to_string(),
                    price: 20,
                    quantity: 0,
                },
                ore: Item {
                    name: "Ore".to_string(),
                    price: 30,
                    quantity: 0,
                },
                ingots: Item {
                    name: "Ingot".to_string(),
                    price: 50,
                    quantity: 0,
                },
                potions: Item {
                    name: "Potion".to_string(),
                    price: 20,
                    quantity: 0,
                },
                rubies: Item {
                    name: "Ruby".to_string(),
                    price: 200,
                    quantity: 0,
                },
                magic_scrolls: Item {
                    name: "Magic Scroll".to_string(),
                    price: 50,
                    quantity: 0,
                },
                bones: Item {
                    name: "Bone".to_string(),
                    price: 50,
                    quantity: 0,
                },
                dragon_hides: Item {
                    name: "Dragon Hide".to_string(),
                    price: 200,
                    quantity: 0,
                },
                runic_tablets: Item {
                    name: "Runic Tablet".to_string(),
                    price: 1000,
                    quantity: 0,
                },
            },
            armor: ArmorInventory {
                leather: Armor {
                    name: "Leather".to_string(),
                    price: 50,
                    owns: false,
                    defense: 10,
                    durability: 100,
                    default_durability: 100,
                    equipped: false,
                },
                bronze: Armor {
                    name: "Bronze".to_string(),
                    price: 200,
                    owns: false,
                    defense: 30,
                    durability: 200,
                    default_durability: 200,
                    equipped: false,
                },
                iron: Armor {
                    name: "Iron".to_string(),
                    price: 500,
                    owns: false,
                    defense: 50,
                    durability: 300,
                    default_durability: 300,
                    equipped: false,
                },
                steel: Armor {
                    name: "Steel".to_string(),
                    price: 750,
                    owns: false,
                    defense: 100,
                    durability: 500,
                    default_durability: 500,
                    equipped: false,
                },
                dragonhide: Armor {
                    name: "Dragonhide".to_string(),
                    price: 1000,
                    owns: false,
                    defense: 200,
                    durability: 500,
                    default_durability: 500,
                    equipped: false,
                },
                mystic: Armor {
                    name: "Magic".to_string(),
                    price: 10000,
                    owns: false,
                    defense: 1000,
                    durability: 10000,
                    default_durability: 10000,
                    equipped: false,
                },
            },
            weapons: WeaponsInventory {
                wooden_sword: Weapon {
                    name: "Wooden Sword".to_string(),
                    price: 10,
                    owns: false,
                    damage: 10,
                    durability: 100,
                    default_durability: 100,
                    equipped: false,
                },
                bronze_sword: Weapon {
                    name: "Bronze Sword".to_string(),
                    price: 30,
                    owns: false,
                    damage: 20,
                    durability: 150,
                    default_durability: 150,
                    equipped: false,
                },
                iron_sword: Weapon {
                    name: "Iron Sword".to_string(),
                    price: 100,
                    owns: false,
                    damage: 50,
                    durability: 200,
                    default_durability: 200,
                    equipped: false,
                },
                steel_sword: Weapon {
                    name: "Steel Rapier".to_string(),
                    price: 500,
                    owns: false,
                    damage: 200,
                    durability: 500,
                    default_durability: 500,
                    equipped: false,
                },
                mystic_sword: Weapon {
                    name: "Magic Sword".to_string(),
                    price: 5000,
                    owns: false,
                    damage: 500,
                    durability: 1000,
                    default_durability: 1000,
                    equipped: false,
                },
                wizard_staff: Weapon {
                    name: "Wizard Staff".to_string(),
                    price: 10000,
                    owns: false,
                    damage: 1000,
                    durability: 2000,
                    default_durability: 2000,
                    equipped: false,
                },
            },
        };

        profile
    }

    /// Creates new instance with filled-in username/password
    /// fields
    pub fn from(username: &str, password: &str) -> UserProfile {
        let mut profile: UserProfile = UserProfile::new();

        profile.settings.username = username.to_string();
        profile.settings.password = password.to_string();

        profile.save();

        profile
    }

    pub fn retrieve(username: &str) -> Result<UserProfile, String> {
        let profile_path: String = file_path(username);
        let mut contents: String = String::new();

        let file_result = files::read(profile_path);

        match file_result {
            Ok(data) => contents = data,
            Err(_) => return Err(format!("Profile '{}' does not exist.", username)),
        }

        match crate::misc::config_encoding::deserialize_user(contents) {
            Ok(user) => Ok(user),
            Err(message) => {
                UserProfile::delete_from_username(username);
                Err(message)
            }
        }
    }

    pub fn save(&self) {
        let serialize_result =
            crate::misc::config_encoding::serialize_user(self).expect("Could not convert user to config file format.");

        let path = files::file_path(&self.settings.username);
        files::write(path, serialize_result)
    }

    pub fn delete_from_username(username: &str) {
        let profile_path: String = file_path(username);

        files::delete(profile_path);
    }

    pub fn delete(&self) {
        UserProfile::delete_from_username(&self.settings.username);
    }
}
