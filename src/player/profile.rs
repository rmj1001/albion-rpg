use super::achievements::*;
use super::armor::*;
use super::bank::*;
use super::equipment::Equipment;
use super::guilds::Guild;
use super::guilds::GuildMemberships;
use super::inventory::*;
use super::settings::Settings;
use super::weapons::*;
use super::xp::*;

use crate::utils::files;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub hp: usize,
    pub hunger: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub health: Health,
    pub xp: XP,
    pub achievements: Achievements,
    pub bank: Bank,
    pub guild_memberships: GuildMemberships,
    pub equipment: Equipment,
    pub inventory: MundaneInventory,
    pub armor: ArmorInventory,
    pub weapons: WeaponsInventory,
    pub settings: Settings,
}

impl Player {
    /// Creates new instance with empty username/password fields
    pub fn new() -> Self {
        let profile: Player = Player {
            settings: Settings {
                username: String::new(),
                password: String::new(),
                developer: false,
                locked: false,
                hardmode: false,
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
            equipment: Equipment {
                armor: None,
                weapon: None,
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
    pub fn from(username: &str, password: &str) -> Player {
        let mut profile: Player = Player::new();

        profile.settings.username = username.to_string();
        profile.settings.password = password.to_string();

        profile.save();

        profile
    }

    pub fn retrieve(username: &str) -> Result<Player, String> {
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

    pub fn save(&self) {
        let serialize_result = crate::utils::files::encoding::serialize_user(self)
            .expect("Could not convert Player to config file format.");

        let path = files::handler::generate_profile_path(&self.settings.username);
        files::handler::write_file(path, serialize_result)
    }

    pub fn delete_from_username(username: &str) {
        let profile_path = files::handler::generate_profile_path(username);

        files::handler::delete_file(profile_path);
    }

    pub fn delete(&self) {
        Player::delete_from_username(&self.settings.username);
    }

    pub fn reset_health(&mut self) {
        self.health.hp = 100;
        self.health.hunger = 0;
    }

    pub fn reset_inventory(&mut self) {
        // Wealth
        self.bank.wallet = 0;

        // Equipment
        self.equipment.armor = None;
        self.equipment.weapon = None;

        // Mundane Inventory
        self.inventory.bait.quantity = 0;
        self.inventory.bones.quantity = 0;
        self.inventory.dragon_hides.quantity = 0;
        self.inventory.fish.quantity = 0;
        self.inventory.food.quantity = 0;
        self.inventory.furs.quantity = 0;
        self.inventory.ingots.quantity = 0;
        self.inventory.magic_scrolls.quantity = 0;
        self.inventory.ore.quantity = 0;
        self.inventory.potions.quantity = 0;
        self.inventory.rubies.quantity = 0;
        self.inventory.runic_tablets.quantity = 0;
        self.inventory.seeds.quantity = 0;
        self.inventory.wood.quantity = 0;

        // Armor
        self.armor = ArmorInventory {
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
        };

        // Weapons
        self.weapons = WeaponsInventory {
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
        }
    }
}
