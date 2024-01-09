use super::profile::UserProfile;
use crate::lib::crypt;
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
    pub fn lock(&mut self) {
        self.locked = true;
    }

    /// Allows profile to login un-hindered.
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    /// Updates developer status
    pub fn set_developer(&mut self, flag: bool) {
        self.developer = flag;
    }

    /// Updates password field
    pub fn change_password(&mut self, new_password: String) {
        let new_hashed_password = crypt::generate(new_password);
        self.password = new_hashed_password;
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
            }

            Err(error) => {
                panic!("I couldn't rename the profile filename: {}", error);
            }
        }
    }
}
