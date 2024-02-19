pub mod encoding {
    use crate::data::player::Player;
    use serde_yaml as encoder;
    use serde_yaml::Error as SerdeError;

    /// Convert TOML to player data
    pub fn decode(data: String) -> Result<Player, String> {
        let user_result: Result<Player, SerdeError> = encoder::from_str(&data);

        match user_result {
            Ok(profile) => Ok(profile),
            Err(message) => Err(format!("This profile is corrupted and will be deleted: {}", message)),
        }
    }

    /// Convert player data to TOML
    pub fn encode(player: &Player) -> Result<String, SerdeError> {
        encoder::to_string(&player)
    }
}

pub mod handler {
    use std::{fs, path::Path};

    pub fn extension() -> &'static str {
        ".albion"
    }

    /// Generates the profile directory path for multiple platforms
    pub fn profile_directory() -> String {
        let os: &str = std::env::consts::OS;

        match os {
            "linux" | "freebsd" | "dragonfly" | "netbsd" | "openbsd" => {
                Path::new(&format!("/home/{}/.anglandia/profiles", whoami::username()))
                    .to_str()
                    .expect("Path could not be converted to string")
                    .to_string()
            }

            "macos" => Path::new(&format!("/Users/{}/.anglandia/profiles", whoami::username()))
                .to_str()
                .expect("Path could not be converted to string")
                .to_string(),

            "windows" => Path::new(&format!(
                r"C:\Users\{}\Documents\anglandia\profiles",
                whoami::username()
            ))
            .to_str()
            .expect("Path could not be converted to string")
            .to_string(),

            _ => panic!("Empty path provided for directory."),
        }
    }

    /// Generates the full path string for profiles depending on platform.
    pub fn generate_profile_path(username: &str) -> String {
        let string: String = format!("{}/{}{}", profile_directory(), username, extension());
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
            panic!("Could not write to disk: {}", message);
        };

        if let Err(message) = fs::write(file_path, data) {
            panic!("Could not write to disk: {}", message);
        }
    }

    /// Read the contents of a file to a string
    pub fn read_file(file_path: String) -> Result<String, String> {
        match fs::read_to_string(file_path.clone()) {
            Ok(contents) => Ok(contents),
            Err(_) => Err(format!("File '{}' does not exist.", file_path)),
        }
    }

    /// Delete a file or panic
    pub fn delete_file(file_path: String) {
        if let Err(error) = fs::remove_file(file_path) {
            panic!("Could not delete profile file: {}", error)
        }
    }
}
