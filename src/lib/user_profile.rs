use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub hitpoints: u32,
    pub hunger: u32,
}

#[derive(Serialize, Deserialize)]
pub struct XP {
    pub combat: u32,
    pub fishing: u32,
    pub cooking: u32,
    pub woodcutting: u32,
    pub mining: u32,
    pub smithing: u32,
    pub thieving: u32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum XPType {
    Combat,
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
    Thieving,
}

impl XP {
    pub fn level(xp: u32) -> u32 {
        (xp / 100) + 1
    }

    pub fn total_xp(&self) -> u32 {
        self.combat
            + self.fishing
            + self.cooking
            + self.woodcutting
            + self.mining
            + self.smithing
            + self.thieving
    }

    pub fn profile_level(&self) -> u32 {
        XP::level(self.total_xp())
    }

    pub fn increment(&mut self, flag: XPType) {
        let more_xp = rand::thread_rng().gen_range(1..5);

        match flag {
            XPType::Combat => self.combat += more_xp,
            XPType::Fishing => self.fishing += more_xp,
            XPType::Cooking => self.cooking += more_xp,
            XPType::Woodcutting => self.woodcutting += more_xp,
            XPType::Mining => self.mining += more_xp,
            XPType::Smithing => self.smithing += more_xp,
            XPType::Thieving => self.thieving += more_xp,
        }
    }

    pub fn get(&self, flag: XPType) -> u32 {
        match flag {
            XPType::Combat => self.combat,
            XPType::Fishing => self.fishing,
            XPType::Cooking => self.cooking,
            XPType::Woodcutting => self.woodcutting,
            XPType::Mining => self.mining,
            XPType::Smithing => self.smithing,
            XPType::Thieving => self.thieving,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub enum ItemNames {
    Fish,
    CookedFish,
    Wood,
    Ore,
    Ingots,
}

#[derive(Serialize, Deserialize)]
pub struct MundaneInventory {
    pub fish: Item,
    pub cooked_fish: Item,
    pub wood: Item,
    pub ore: Item,
    pub ingots: Item,
}

#[derive(Serialize, Deserialize)]
pub struct Armor {
    pub name: String,
    pub price: u32,
    pub owns: bool,
    pub defense: u16,
    pub durability: i16,
    pub default_durability: i16,
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub price: u32,
    pub owns: bool,
    pub damage: u16,
    pub durability: i16,
    default_durability: i16,
}

impl Weapon {
    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability <= 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

impl Armor {
    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability <= 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArmorInventory {
    pub leather: Armor,
    pub bronze: Armor,
    pub iron: Armor,
    pub steel: Armor,
    pub dragonhide: Armor,
    pub mystic: Armor,
}

#[derive(Serialize, Deserialize)]
pub struct WeaponsInventory {
    pub wooden_sword: Weapon,
    pub bronze_sword: Weapon,
    pub iron_sword: Weapon,
    pub steel_sword: Weapon,
    pub mystic_sword: Weapon,
    pub wizard_staff: Weapon,
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
            inventory: MundaneInventory {
                fish: Item {
                    name: "Fish".to_string(),
                    price: 10,
                    quantity: 0,
                },
                cooked_fish: Item {
                    name: "Cooked Fish".to_string(),
                    price: 25,
                    quantity: 0,
                },
                wood: Item {
                    name: "Logs".to_string(),
                    price: 20,
                    quantity: 0,
                },
                ore: Item {
                    name: "Ore".to_string(),
                    price: 30,
                    quantity: 0,
                },
                ingots: Item {
                    name: "Ingots".to_string(),
                    price: 50,
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

    /// Lists all profiles registered with the game and removes the .json from the filename.
    pub fn list_profiles() -> Vec<String> {
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

    fn retrieve_profile_to_string(username: &str) -> Option<String> {
        let profile_path: String = UserProfile::profile_path(username);
        let file_path: &Path = Path::new(&profile_path);

        match fs::read_to_string(file_path) {
            Ok(contents) => Some(contents),
            Err(_) => None,
        }
    }

    /// Retrieves a profile from a config file. If no profile is retrieved then the login handler
    /// will handle the result
    pub fn retrieve_profile(username: &str) -> Option<UserProfile> {
        let profile_contents = UserProfile::retrieve_profile_to_string(username);

        if profile_contents.is_some() {
            match json::from_str(&profile_contents.unwrap()) {
                Ok(profile) => Some(profile),
                Err(_) => {
                    println!("\nThis user profile is corrupted and will be deleted.");
                    UserProfile::delete_profile(username);

                    None
                }
            }
        } else {
            None
        }
    }

    /// API for deleting a profile based on a username string
    pub fn delete_profile(username: &str) {
        let profile_path_string: String = UserProfile::profile_path(username);
        let profile_path = Path::new(&profile_path_string);

        match fs::remove_file(profile_path) {
            Ok(_) => {}
            Err(error) => panic!("Could not delete profile file: {}", error),
        }
    }

    /// Deletes the profile file and logs out
    pub fn self_delete_profile(&self) {
        UserProfile::delete_profile(&self.username);
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
