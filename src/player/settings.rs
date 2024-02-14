use super::profile::Player;
use crate::utils::{crypt, files};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub username: String,
    pub password: String,
    pub developer: bool,
    pub locked: bool,
    pub hardmode: bool,
}

impl Settings {
    /// Hinders profile login without double password entry
    pub fn lock(player: &mut Player) {
        player.settings.locked = true;
        player.save();
    }

    /// Allows profile to login un-hindered.
    pub fn unlock(player: &mut Player) {
        player.settings.locked = false;
        player.save();
    }

    /// Updates developer status
    pub fn set_developer(player: &mut Player, flag: bool) {
        player.settings.developer = flag;
        player.save();
    }

    /// Updates password field
    pub fn change_password(player: &mut Player, new_password: String) {
        let new_hashed_password = crypt::generate_hash(new_password);
        player.settings.password = new_hashed_password;
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

    pub fn toggle_hardmode(player: &mut Player) {
        player.settings.hardmode = !player.settings.hardmode;
    }
}
