use crate::{data::player::Player, panic_menu, prelude::*};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs, path::Path};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Settings {
    pub username: String,
    pub password_hash: String,
    pub developer: bool,
    pub hardmode: bool,
}

impl Settings {
    pub fn new<T: Display, U: Display>(username: &T, password_hash: &U) -> Self {
        Self {
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(&self.username, &self.password_hash);
    }

    /// Either reset inventory or delete profile if defeated in battle
    pub fn toggle_hardmode(player: &mut Player) {
        player.settings.hardmode = !player.settings.hardmode;
        player.save();

        if player.settings.hardmode {
            success(Some("Hardmode enabled."));
        } else {
            success(Some("Hardmode disabled."));
        }
    }

    /// Updates developer status
    pub fn toggle_developer(player: &mut Player) {
        player.settings.developer = !player.settings.developer;
        player.save();

        if player.settings.developer {
            player.achievements.hacked_the_game = true;
            success(Some("Developer mode enabled."));
        } else {
            success(Some("Developer mode disabled."));
        }
    }

    /// Updates password field
    pub fn change_password(player: &mut Player, new_password: String) {
        let new_hashed_password = generate_hash(&new_password);
        player.settings.password_hash = new_hashed_password;
        player.save();
    }

    /// Updates the username field and profile file name.
    pub fn change_username(player: &mut Player, new_username: String) {
        let old_profile_path = player_file_path(&player.settings.username);
        let old_file_path: &Path = Path::new(&old_profile_path);

        let new_profile_path = player_file_path(&new_username);
        let new_file_path: &Path = Path::new(&new_profile_path);

        match fs::rename(old_file_path, new_file_path) {
            Ok(_) => {
                player.settings.username = new_username;
            }

            Err(error) => {
                panic_menu!("I couldn't rename the profile filename: {}", error);
            }
        }

        player.save();
    }
}
