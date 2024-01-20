use super::profile::UserProfile;
use crate::lib::{crypt, files};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub username: String,
    pub password: String,
    pub developer: bool,
    pub locked: bool,
}

impl Settings {
    /// Hinders profile login without double password entry
    pub fn lock(user: &mut UserProfile) {
        user.settings.locked = true;
        user.save();
    }

    /// Allows profile to login un-hindered.
    pub fn unlock(user: &mut UserProfile) {
        user.settings.locked = false;
        user.save();
    }

    /// Updates developer status
    pub fn set_developer(user: &mut UserProfile, flag: bool) {
        user.settings.developer = flag;
        user.save();
    }

    /// Updates password field
    pub fn change_password(user: &mut UserProfile, new_password: String) {
        let new_hashed_password = crypt::generate_hash(new_password);
        user.settings.password = new_hashed_password;
        user.save();
    }

    /// Updates the username field and profile file name.
    pub fn change_username(user: &mut UserProfile, new_username: String) {
        let old_profile_path: String = files::file_path(&user.settings.username);
        let old_file_path: &Path = Path::new(&old_profile_path);

        let new_profile_path: String = files::file_path(&new_username);
        let new_file_path: &Path = Path::new(&new_profile_path);

        match fs::rename(old_file_path, new_file_path) {
            Ok(_) => {
                user.settings.username = new_username;
            }

            Err(error) => {
                panic!("I couldn't rename the profile filename: {}", error);
            }
        }

        user.save();
    }
}
