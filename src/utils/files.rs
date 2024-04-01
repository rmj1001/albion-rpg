/*!
Helper functionality for reading/writing player data files.
*/
use crate::{
    panic_menu,
    prelude::{ProfileError, Result},
};
use std::env::consts::OS;
use std::{fmt::Display, fs, path::Path};

pub const FOLDER_NAME: &str = ".albion_term_rpg";
pub const EXTENSION: &str = "albion";

/**
Generate the path for the folder containing player files.

# Example

```
use albion_terminal_rpg::prelude::player_files_directory;

let dir = player_files_directory();
```
*/
pub fn player_files_directory() -> String {
    match OS {
        "linux" | "freebsd" | "dragonfly" | "netbsd" | "openbsd" => {
            Path::new(&format!("/home/{}/{}/profiles", whoami::username(), FOLDER_NAME))
                .to_str()
                .expect("Path could not be converted to string")
                .to_string()
        }

        "macos" => Path::new(&format!("/Users/{}/{}/profiles", whoami::username(), FOLDER_NAME))
            .to_str()
            .expect("Path could not be converted to string")
            .to_string(),

        "windows" => Path::new(&format!(
            r"C:\Users\{}\Documents\{}\profiles",
            whoami::username(),
            FOLDER_NAME
        ))
        .to_str()
        .expect("Path could not be converted to string")
        .to_string(),

        _ => panic_menu!("Empty path provided for directory."),
    }
}

/**
Generates the full path string for player data files.

# Example

```
use albion_terminal_rpg::prelude::player_file_path;

let file = player_file_path("Steve");
```
*/
pub fn player_file_path<T: Display>(username: T) -> String {
    let player_file_path: String = format!("{}/{}.{}", player_files_directory(), username, EXTENSION);

    Path::new(&player_file_path)
        .to_str()
        .expect("Path could not be converted to string")
        .to_string()
}

/**
Lists all profiles registered with the game and removes the extension from the filename.

# Example

```ignore
use albion_terminal_rpg::prelude::all_profiles;

let files: Vec<String> = all_profiles();
```
*/
pub fn all_profiles() -> Vec<String> {
    match fs::read_dir(player_files_directory()) {
        Ok(directory_read) => directory_read
            .filter(|file_result| {
                file_result
                    .as_ref()
                    .expect("Failed to list files.")
                    .file_name()
                    .to_str()
                    .unwrap_or("")
                    .to_string()
                    .contains(EXTENSION)
            })
            .map(|file| {
                file.expect("Failed to list files.")
                    .file_name()
                    .to_str()
                    .unwrap_or("")
                    .to_string()
                    .replace(&format!(".{EXTENSION}"), "")
            })
            .collect(),
        Err(error) => panic_menu!("Could not read the directory: {}", error),
    }
}

/**
Write data to a file.

# Example

```ignore
use albion_terminal_rpg::prelude::write_file;

write_file("/home/{user}/albion_terminal_rpg/player.albion", "some data");
```
*/
pub fn write_file<T: Display>(file_path: &str, data: T) {
    if let Err(message) = fs::create_dir_all(player_files_directory()) {
        panic_menu!(format!(
            "Could create directory on disk for player save data:\n{}",
            message
        ));
    };

    if let Err(message) = fs::write(file_path, data.to_string()) {
        panic_menu!("Could not write to '{}':\n{}", file_path, message);
    }
}

/**
Read the contents of a file to a string.

# Example

```ignore
use albion_terminal_rpg::prelude::{Result, read_file};

let contents: Result<String> = read_file("/home/{user}/albion_terminal_rpg/player.albion");
```
*/
pub fn read_file<T: Display>(file_path: &T) -> Result<String> {
    match fs::read_to_string(file_path.to_string()) {
        Ok(data) => Ok(data),
        Err(_) => Err(Box::new(ProfileError::Corrupted)),
    }
}

/**
Delete a file or panic

# Example

```ignore
use albion_terminal_rpg::prelude::delete_file;

delete_file("/home/{user}/albion_terminal_rpg/player.albion");
```
*/
pub fn delete_file<T: Display>(file_path: &T) {
    if let Err(error) = fs::remove_file(file_path.to_string()) {
        panic_menu!(format!("Could not delete profile file:\n{}", error))
    }
}
