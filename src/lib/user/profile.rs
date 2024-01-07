use crate::lib::user::armor::*;
use crate::lib::user::bank::*;
use crate::lib::user::inventory::*;
use crate::lib::user::weapons::*;
use crate::lib::user::xp::*;

use serde::{Deserialize, Serialize};
use serde_json as json;
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub hitpoints: u32,
    pub hunger: u32,
}

#[allow(clippy::large_enum_variant)]
pub enum ProfileRetrievalResult {
    Some(UserProfile),
    None(String),
}

pub enum JSONResult {
    Some(String),
    None,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub password: String,
    pub locked: bool,
    pub is_developer: bool,
    pub health: Health,
    pub xp: XP,
    pub gold: u32,
    pub bank: Bank,
    pub inventory: MundaneInventory,
    pub armor: ArmorInventory,
    pub weapons: WeaponsInventory,
}

impl UserProfile {
    /// Creates new instance with empty username/password fields
    pub fn new() -> Self {
        let profile: UserProfile = UserProfile {
            username: String::new(),
            password: String::new(),
            locked: false,
            is_developer: false,
            health: Health {
                hitpoints: 100,
                hunger: 0,
            },
            xp: XP {
                combat: 0,
                fishing: 0,
                cooking: 0,
                woodcutting: 0,
                mining: 0,
                smithing: 0,
                thieving: 0,
            },
            gold: 0,
            bank: Bank {
                account1: 0,
                account2: 0,
                account3: 0,
                account4: 0,
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
                },
                bronze: Armor {
                    name: "Bronze".to_string(),
                    price: 200,
                    owns: false,
                    defense: 30,
                    durability: 200,
                    default_durability: 200,
                },
                iron: Armor {
                    name: "Iron".to_string(),
                    price: 500,
                    owns: false,
                    defense: 50,
                    durability: 300,
                    default_durability: 300,
                },
                steel: Armor {
                    name: "Steel".to_string(),
                    price: 750,
                    owns: false,
                    defense: 100,
                    durability: 500,
                    default_durability: 500,
                },
                dragonhide: Armor {
                    name: "Dragonhide".to_string(),
                    price: 1000,
                    owns: false,
                    defense: 200,
                    durability: 500,
                    default_durability: 500,
                },
                mystic: Armor {
                    name: "Magic".to_string(),
                    price: 10000,
                    owns: false,
                    defense: 1000,
                    durability: 10000,
                    default_durability: 10000,
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
                },
                bronze_sword: Weapon {
                    name: "Bronze Sword".to_string(),
                    price: 30,
                    owns: false,
                    damage: 20,
                    durability: 150,
                    default_durability: 150,
                },
                iron_sword: Weapon {
                    name: "Iron Sword".to_string(),
                    price: 100,
                    owns: false,
                    damage: 50,
                    durability: 200,
                    default_durability: 200,
                },
                steel_sword: Weapon {
                    name: "Steel Rapier".to_string(),
                    price: 500,
                    owns: false,
                    damage: 200,
                    durability: 500,
                    default_durability: 500,
                },
                mystic_sword: Weapon {
                    name: "Magic Sword".to_string(),
                    price: 5000,
                    owns: false,
                    damage: 500,
                    durability: 1000,
                    default_durability: 1000,
                },
                wizard_staff: Weapon {
                    name: "Wizard Staff".to_string(),
                    price: 10000,
                    owns: false,
                    damage: 1000,
                    durability: 2000,
                    default_durability: 2000,
                },
            },
        };

        profile
    }

    /// Creates new instance with filled-in username/password
    /// fields
    pub fn from(username: &str, password: &str) -> UserProfile {
        let mut profile: UserProfile = UserProfile::new();

        profile.username = username.to_string();
        profile.password = password.to_string();

        profile.save();

        profile
    }

    /// Writes the current state of the UserProfile to a JSON string
    pub fn to_pretty_json(&self) -> String {
        match json::to_string_pretty(self) {
            Ok(json_string) => json_string,
            Err(error) => panic!("Could not serialize player data to JSON: {}", error),
        }
    }

    /// Generates the profile directory path for multiple platforms
    pub fn directory_path() -> String {
        let os: &str = std::env::consts::OS;
        let mut directory_path: String = String::new();

        match os {
            "linux" => directory_path = format!("/home/{}/.anglandia/profiles", whoami::username()),

            "macos" => {
                directory_path = format!("/Users/{}/.anglandia/profiles", whoami::username())
            }

            "freebsd" => {
                directory_path = format!("/home/{}/.anglandia/profiles", whoami::username())
            }

            "dragonfly" => {
                directory_path = format!("/home/{}/.anglandia/profiles", whoami::username())
            }

            "netbsd" => {
                directory_path = format!("/home/{}/.anglandia/profiles", whoami::username())
            }

            "openbsd" => {
                directory_path = format!("/home/{}/.anglandia/profiles", whoami::username())
            }

            _ => {}
        }

        directory_path
    }

    /// Generates the full path string for profiles depending on platform.
    pub fn file_path(username: &str) -> String {
        format!("{}/{}.json", UserProfile::directory_path(), username)
    }

    /// Lists all profiles registered with the game and removes the .json from the filename.
    pub fn list_all() -> Vec<String> {
        let directory = UserProfile::directory_path();
        let files_result = fs::read_dir(directory);

        match files_result {
            Ok(directory_read) => {
                let files = directory_read.filter(|file_result| {
                    file_result
                        .as_ref()
                        .expect("Could not list files")
                        .file_name()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .contains(".json")
                });

                files
                    .map(|file| {
                        file.unwrap()
                            .file_name()
                            .to_str()
                            .unwrap()
                            .to_string()
                            .replace(".json", "")
                    })
                    .collect()
            }
            Err(error) => panic!("Could not read the directory: {}", error),
        }
    }

    /// Writes the UserProfile data to a config file.
    /// If the file exists, it is overwritten with the current profile state.
    /// If the file does not exist, the default values are written to the file.
    pub fn save(&self) {
        let path_string: String = UserProfile::file_path(&self.username);

        match fs::create_dir_all(UserProfile::directory_path()) {
            Ok(_) => {}
            Err(error) => panic!(
                "Could not write profile to disk or create parent folders: {}",
                error
            ),
        }

        if !Path::new(&path_string).exists() {
            match fs::write(&path_string, UserProfile::new().to_pretty_json()) {
                Ok(_) => {}
                Err(write_error) => panic!("Could not write profile to disk: {}", write_error),
            }
        }

        match fs::write(path_string, self.to_pretty_json()) {
            Ok(_) => {}
            Err(error) => panic!("Could not write profile to disk: {}", error),
        }
    }

    /// Retrieves a profile from a config file. If no profile is retrieved then the login handler
    /// will handle the result
    pub fn retrieve(username: &str) -> ProfileRetrievalResult {
        let profile_path: String = UserProfile::file_path(username);
        let file_path: &Path = Path::new(&profile_path);
        let mut profile_contents: String = String::new();

        match fs::read_to_string(file_path) {
            Ok(contents) => profile_contents = contents,
            Err(_) => {
                return ProfileRetrievalResult::None(format!(
                    "User profile '{}' does not exist.",
                    username
                ));
            }
        }

        match json::from_str(&profile_contents) {
            Ok(profile) => ProfileRetrievalResult::Some(profile),
            Err(_) => {
                UserProfile::delete_from_username(username);

                ProfileRetrievalResult::None(
                    "This user profile is corrupted and will be deleted.".to_string(),
                )
            }
        }
    }

    /// API for deleting a profile based on a username string
    pub fn delete_from_username(username: &str) {
        let profile_path_string: String = UserProfile::file_path(username);
        let profile_path = Path::new(&profile_path_string);

        match fs::remove_file(profile_path) {
            Ok(_) => {}
            Err(error) => panic!("Could not delete profile file: {}", error),
        }
    }

    /// Deletes the profile file and logs out
    pub fn delete(&self) {
        UserProfile::delete_from_username(&self.username);
    }

    /// Hinders profile login without double password entry
    pub fn lock(&mut self) {
        self.locked = true;
        self.save();
    }

    /// Allows profile to login un-hindered.
    pub fn unlock(&mut self) {
        self.locked = false;
        self.save();
    }

    /// Updates developer status
    pub fn set_developer(&mut self, flag: bool) {
        self.is_developer = flag;
        self.save();
    }

    /// Updates password field
    pub fn change_password(&mut self, new_password: String) {
        self.password = new_password;
        self.save();
    }

    /// Updates the username field and profile file name.
    pub fn change_username(&mut self, new_username: String) {
        let old_profile_path: String = UserProfile::file_path(&self.username);
        let old_file_path: &Path = Path::new(&old_profile_path);

        let new_profile_path: String = UserProfile::file_path(&new_username);
        let new_file_path: &Path = Path::new(&new_profile_path);

        match fs::rename(old_file_path, new_file_path) {
            Ok(_) => {
                self.username = new_username;
                self.save();
            }

            Err(error) => {
                panic!("I couldn't rename the profile filename: {}", error);
            }
        }
    }
}
