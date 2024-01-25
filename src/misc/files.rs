use std::fs;

/// Generates the profile directory path for multiple platforms
pub fn directory_path() -> String {
    let os: &str = std::env::consts::OS;
    let mut directory_path: String = String::new();

    match os {
        "linux" => directory_path = format!("/home/{}/.anglandia/profiles", whoami::username()),

        "macos" => directory_path = format!("/Users/{}/.anglandia/profiles", whoami::username()),

        "freebsd" => directory_path = format!("/home/{}/.anglandia/profiles", whoami::username()),

        "dragonfly" => directory_path = format!("/home/{}/.anglandia/profiles", whoami::username()),

        "netbsd" => directory_path = format!("/home/{}/.anglandia/profiles", whoami::username()),

        "openbsd" => directory_path = format!("/home/{}/.anglandia/profiles", whoami::username()),

        _ => {}
    }

    directory_path
}

/// Generates the full path string for profiles depending on platform.
pub fn file_path(username: &str) -> String {
    format!("{}/{}.toml", directory_path(), username)
}

/// Lists all profiles registered with the game and removes the .json from the filename.
pub fn list_all() -> Vec<String> {
    let directory = directory_path();
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
                    .contains(".toml")
            })
            .map(|file| {
                file.expect("Failed to list files.")
                    .file_name()
                    .to_str()
                    .unwrap_or("")
                    .to_string()
                    .replace(".toml", "")
            })
            .collect(),
        Err(error) => panic!("Could not read the directory: {}", error),
    }
}

/// Writes the data to a file.
/// If the file exists, it is overwritten.
/// If the file does not exist, the default values are written to the file.
pub fn write(file_path: String, data: String) {
    if let Err(message) = fs::create_dir_all(directory_path()) {
        panic!("Could not write to disk: {}", message);
    };

    if let Err(message) = fs::write(file_path, data) {
        panic!("Could not write to disk: {}", message);
    }
}

pub fn read(file_path: String) -> Result<String, String> {
    match fs::read_to_string(file_path.clone()) {
        Ok(contents) => Ok(contents),
        Err(_) => Err(format!("File '{}' does not exist.", file_path)),
    }
}

pub fn delete(file_path: String) {
    match fs::remove_file(file_path) {
        Ok(_) => {}
        Err(error) => panic!("Could not delete profile file: {}", error),
    }
}
