use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub hitpoints: u32,
    pub hunger: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub copper: u32,
    pub silver: u32,
    pub gold: u32,
    pub electrum: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub xp: u32,
    pub level: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Guilds {
    pub fishing: Job,
    pub cooking: Job,
    pub woodcutting: Job,
    pub mining: Job,
    pub smithing: Job,
    pub thieving: Job,
}

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub fish: u16,
    pub cooked_fish: u16,
    pub wood: u16,
    pub ore: u16,
    pub ingots: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Armor {
    pub defense: u16,
    pub durability: u16,
}

#[derive(Serialize, Deserialize)]
pub struct ArmorInventory {
    pub leather: Option<Armor>,
    pub bronze: Option<Armor>,
    pub iron: Option<Armor>,
    pub steel: Option<Armor>,
    pub dragonhide: Option<Armor>,
    pub mystic: Option<Armor>,
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub damage: u16,
    pub durability: u16,
}

#[derive(Serialize, Deserialize)]
pub struct WeaponsInventory {
    pub wooden_sword: Option<Weapon>,
    pub bronze_sword: Option<Weapon>,
    pub iron_sword: Option<Weapon>,
    pub steel_sword: Option<Weapon>,
    pub mystic_sword: Option<Weapon>,
    pub wizard_staff: Option<Weapon>,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub password: String,
    pub locked: bool,
    pub is_developer: bool,
    pub wallet: Wallet,
    pub health: Health,
    pub guilds: Guilds,
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
            wallet: Wallet {
                copper: 0,
                silver: 0,
                gold: rand::prelude::thread_rng().gen_range(75..200),
                electrum: 0,
            },
            health: Health {
                hitpoints: 100,
                hunger: 0,
            },
            guilds: Guilds {
                fishing: Job { xp: 0, level: 1 },
                cooking: Job { xp: 0, level: 1 },
                woodcutting: Job { xp: 0, level: 1 },
                mining: Job { xp: 0, level: 1 },
                smithing: Job { xp: 0, level: 1 },
                thieving: Job { xp: 0, level: 1 },
            },
            armor: ArmorInventory {
                leather: None,
                bronze: None,
                iron: None,
                steel: None,
                dragonhide: None,
                mystic: None,
            },
            weapons: WeaponsInventory {
                wooden_sword: None,
                bronze_sword: None,
                iron_sword: None,
                steel_sword: None,
                mystic_sword: None,
                wizard_staff: None,
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

        profile.save_profile();

        profile
    }

    /// Writes the current state of the UserProfile to a JSON string
    fn write_to_json(&self) -> String {
        match json::to_string(self) {
            Ok(json_string) => json_string,
            Err(error) => panic!("Could not serialize player data to JSON: {}", error),
        }
    }

    /// Creates a UserProfile struct from a serialized JSON string
    fn read_from_json(data: String) -> UserProfile {
        match json::from_str(&data) {
            Ok(profile) => profile,
            Err(error) => panic!("Could not deserialize player data from JSON: {}", error),
        }
    }

    /// Generates the profile directory path for multiple platforms
    fn directory_path() -> String {
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
    fn profile_path(username: &str) -> String {
        format!("{}/{}.json", UserProfile::directory_path(), username)
    }

    /// Writes the UserProfile data to a config file.
    /// If the file exists, it is overwritten with the current profile state.
    /// If the file does not exist, the default values are written to the file.
    pub fn save_profile(&self) {
        let path_string: String = UserProfile::profile_path(&self.username);

        match fs::create_dir_all(UserProfile::directory_path()) {
            Ok(_) => {}
            Err(error) => panic!(
                "Could not write profile to disk or create parent folders: {}",
                error
            ),
        }

        if !Path::new(&path_string).exists() {
            match fs::write(&path_string, UserProfile::new().write_to_json()) {
                Ok(_) => {}
                Err(write_error) => panic!("Could not write profile to disk: {}", write_error),
            }
        }

        match fs::write(path_string, self.write_to_json()) {
            Ok(_) => {}
            Err(error) => panic!("Could not write profile to disk: {}", error),
        }
    }

    /// Retrieves a profile from a config file. If no profile is retrieved then the login handler
    /// will handle the result
    pub fn retrieve_profile(username: &str) -> Option<UserProfile> {
        let profile_path: String = UserProfile::profile_path(username);
        let file_path: &Path = Path::new(&profile_path);

        match fs::read_to_string(file_path) {
            Ok(contents) => {
                #[allow(unused_mut)]
                let mut profile = UserProfile::read_from_json(contents);
                Some(profile)
            }
            Err(_) => None,
        }
    }

    /// Deletes the profile file and logs out
    pub fn delete_profile(&self, optional_username: Option<&str>) {
        let mut profile_path: String = String::new();

        match optional_username {
            Some(name) => {
                profile_path = UserProfile::profile_path(name);
            }

            None => {
                profile_path = UserProfile::profile_path(&self.username);
            }
        }

        let file_path: &Path = Path::new(&profile_path);

        match fs::remove_file(file_path) {
            Ok(_) => {}
            Err(error) => panic!("Could not delete profile file: {}", error),
        }
    }

    /// Hinders profile login without double password entry
    pub fn lock(&mut self) {
        self.locked = true;
        self.save_profile();
    }

    /// Allows profile to login un-hindered.
    pub fn unlock(&mut self) {
        self.locked = false;
        self.save_profile();
    }

    /// Updates developer status
    pub fn developer_mode(&mut self, flag: bool) {
        self.is_developer = flag;
        self.save_profile();
    }

    /// Updates password field
    pub fn change_password(&mut self, new_password: String) {
        self.password = new_password;
        self.save_profile();
    }

    /// Updates the username field and profile file name.
    pub fn change_username(&mut self, new_username: String) {
        let old_profile_path: String = UserProfile::profile_path(&self.username);
        let old_file_path: &Path = Path::new(&old_profile_path);

        let new_profile_path: String = UserProfile::profile_path(&new_username);
        let new_file_path: &Path = Path::new(&new_profile_path);

        match fs::rename(old_file_path, new_file_path) {
            Ok(_) => {
                self.username = new_username;
                self.save_profile();
            }

            Err(error) => {
                panic!("I couldn't rename the profile filename: {}", error);
            }
        }
    }
}
