pub mod encoding {
    use crate::data::player::Player;
    use crate::{DataError, ProfileError};
    use serde_yaml as encoder;
    use serde_yaml::Error as SerdeError;

    /// Convert TOML to player data
    pub fn decode(data: String) -> Result<Player, ProfileError> {
        let user_result: Result<Player, SerdeError> = encoder::from_str(&data);

        match user_result {
            Ok(profile) => Ok(profile),
            Err(_) => Err(crate::ProfileError::Corrupted),
        }
    }

    /// Convert player data to TOML
    pub fn encode(player: &Player) -> Result<String, DataError> {
        match encoder::to_string(&player) {
            Ok(string) => Ok(string),
            Err(_) => Err(DataError::Encode),
        }
    }
}

pub mod handler {
    use std::{fs, path::Path};

    use crate::ProfileError;

    pub fn folder_name() -> &'static str {
        "albion_term_rpg"
    }

    pub fn extension() -> &'static str {
        "albion"
    }

    /// Generates the profile directory path for multiple platforms
    pub fn profile_directory() -> String {
        let os: &str = std::env::consts::OS;

        match os {
            "linux" | "freebsd" | "dragonfly" | "netbsd" | "openbsd" => {
                Path::new(&format!("/home/{}/{}/profiles", whoami::username(), folder_name()))
                    .to_str()
                    .expect("Path could not be converted to string")
                    .to_string()
            }

            "macos" => Path::new(&format!("/Users/{}/{}/profiles", whoami::username(), folder_name()))
                .to_str()
                .expect("Path could not be converted to string")
                .to_string(),

            "windows" => Path::new(&format!(
                r"C:\Users\{}\Documents\{}\profiles",
                whoami::username(),
                folder_name()
            ))
            .to_str()
            .expect("Path could not be converted to string")
            .to_string(),

            _ => panic!("Empty path provided for directory."),
        }
    }

    /// Generates the full path string for profiles depending on platform.
    pub fn generate_profile_path(username: &str) -> String {
        let string: String = format!("{}/{}.{}", profile_directory(), username, extension());
        Path::new(&string)
            .to_str()
            .expect("Path could not be converted to string")
            .to_string()
    }

    /// Lists all profiles registered with the game and removes the .json from the filename.
    pub fn list_all_profiles() -> Vec<String> {
        let directory = profile_directory();

        let files_result = fs::read_dir(directory);

        match files_result {
            Ok(directory_read) => directory_read
                .filter(|file_result| {
                    file_result
                        .as_ref()
                        .expect("Failed to list files.")
                        .file_name()
                        .to_str()
                        .unwrap_or("")
                        .to_string()
                        .contains(extension())
                })
                .map(|file| {
                    file.expect("Failed to list files.")
                        .file_name()
                        .to_str()
                        .unwrap_or("")
                        .to_string()
                        .replace(extension(), "")
                })
                .collect(),
            Err(error) => panic!("Could not read the directory: {}", error),
        }
    }

    /// Writes the data to a file.
    /// If the file exists, it is overwritten.
    /// If the file does not exist, the default values are written to the file.
    pub fn write_file(file_path: String, data: String) {
        let directory = profile_directory();

        if let Err(message) = fs::create_dir_all(directory) {
            panic!("Could create directory on disk for player save data:\n{}", message);
        };

        if let Err(message) = fs::write(&file_path, data) {
            panic!("Could not write to '{}':\n{}", file_path, message);
        }
    }

    /// Read the contents of a file to a string
    pub fn read_file(file_path: String) -> Result<String, ProfileError> {
        match fs::read_to_string(file_path.clone()) {
            Ok(contents) => Ok(contents),
            Err(_) => Err(crate::ProfileError::DoesNotExist),
        }
    }

    /// Delete a file or panic
    pub fn delete_file(file_path: String) {
        if let Err(error) = fs::remove_file(file_path) {
            panic!("Could not delete profile file:\n{}", error)
        }
    }
}
