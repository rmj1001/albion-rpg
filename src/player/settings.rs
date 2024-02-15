use super::profile::Player;
use crate::utils::{crypt, files, messages::success_msg};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub username: String,
    pub password_hash: String,
    pub developer: bool,
    pub locked: bool,
    pub hardmode: bool,
}

impl Settings {
    pub fn new(username: &str, password_hash: &str) -> Self {
        Self {
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            developer: false,
            locked: false,
            hardmode: false,
        }
    }

    /// Hinders profile login without double password entry
    pub fn toggle_lock(player: &mut Player) {
        player.settings.locked = !player.settings.locked;
        player.save();

        if player.settings.locked {
            success_msg("Player locked.");
        } else {
            success_msg("Player unlocked.");
        }
    }

    /// Either reset inventory or delete profile if defeated in battle
    pub fn toggle_hardmode(player: &mut Player) {
        player.settings.hardmode = !player.settings.hardmode;
        player.save();

        if player.settings.hardmode {
            success_msg("Hardmode enabled.");
        } else {
            success_msg("Hardmode disabled.");
        }
    }

    /// Updates developer status
    pub fn toggle_developer(player: &mut Player) {
        player.settings.developer = !player.settings.developer;
        player.save();

        if player.settings.developer {
            success_msg("Developer mode enabled.");
        } else {
            success_msg("Developer mode disabled.");
        }
    }

    /// Updates password field
    pub fn change_password(player: &mut Player, new_password: String) {
        let new_hashed_password = crypt::generate_hash(new_password);
        player.settings.password_hash = new_hashed_password;
        player.save();
    }

    /// Updates the username field and profile file name.
    pub fn change_username(player: &mut Player, new_username: String) {
        let old_profile_path = files::handler::generate_profile_path(&player.settings.username);
        let old_file_path: &Path = Path::new(&old_profile_path);

        let new_profile_path = files::handler::generate_profile_path(&new_username);
        let new_file_path: &Path = Path::new(&new_profile_path);

        match fs::rename(old_file_path, new_file_path) {
            Ok(_) => {
                player.settings.username = new_username;
            }

            Err(error) => {
                panic!("I couldn't rename the profile filename: {}", error);
            }
        }

        player.save();
    }
}
