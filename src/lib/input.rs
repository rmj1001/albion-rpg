use std::str::FromStr;

use crate::lib::tui::press_enter_to_continue;

/// Returns the index of the element in the vector selected.
pub fn selector(options: &[&str], default_index: usize, optional_prompt: Option<&str>) -> usize {
    match optional_prompt {
        Some(prompt) => dialoguer::Select::new()
            .with_prompt(prompt)
            .items(options)
            .default(0)
            .interact()
            .unwrap_or(default_index),
        None => dialoguer::Select::new()
            .items(options)
            .default(0)
            .interact()
            .unwrap_or(default_index),
    }
}

pub fn select_from_vector(
    options: Vec<String>,
    default_index: usize,
    optional_prompt: Option<&str>,
) -> usize {
    match optional_prompt {
        Some(prompt) => dialoguer::Select::new()
            .with_prompt(prompt)
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap_or(default_index),
        None => dialoguer::Select::new()
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap_or(default_index),
    }
}

pub fn prompt_input(prompt: &str) -> String {
    let input_result = dialoguer::Input::new().with_prompt(prompt).interact_text();

    match input_result {
        Ok(input) => input,
        Err(error) => panic!("Dialoguer input failed: {}", error),
    }
}

/// Attempts to cast the string to a generic type
pub fn input_generic<T>(prompt: &str) -> Result<T, &str>
where
    T: FromStr,
{
    let input_string = prompt_input(prompt);
    let trimmed = input_string.trim();

    match trimmed.parse::<T>() {
        Ok(out) => Ok(out),
        Err(_) => {
            invalid_input(Some(&input_string), None, false);
            Err("")
        }
    }
}

/// "y" and "yes" return true. "n" and "no" return false.
pub fn yes_or_no(prompt: &str) -> bool {
    loop {
        let input = prompt_input(&format!("{} (y/n)", prompt)).to_lowercase();

        match &input[..] {
            "y" => return true,
            "yes" => return true,
            "n" => return false,
            "no" => return false,
            text => {
                invalid_input(Some(text), Some("'yes' or 'no'"), true);
                continue;
            }
        };
    }
}

pub fn password() -> String {
    let dialoguer_result = dialoguer::Password::new()
        .with_prompt("Password")
        .interact();

    match dialoguer_result {
        Ok(password) => password,
        Err(error) => panic!("Failed to read password with dialogue: {}", error),
    }
}

/// Standard panic message for dialogue selector
pub fn out_of_bounds(optional_error: Option<&str>) {
    match optional_error {
        Some(error) => panic!("Dialogue selected index out of option's bounds: {}", error),
        None => panic!("Dialogue selected index out of option's bounds."),
    }
}

/// input: The invalid input
///
/// expected: The expected input
///
/// pause: Ask the user to press enter to continue?
pub fn invalid_input(input: Option<&str>, expected: Option<&str>, pause: bool) {
    let mut output_string = String::new();

    match input {
        Some(text) => output_string.push_str(&format!("Invalid input '{}'.", text)),
        None => output_string.push_str("Invalid input."),
    }

    if let Some(text) = expected {
        output_string.push_str(&format!(" Expected '{}'.", text));
    }

    println!("{}", output_string);

    if pause {
        press_enter_to_continue();
    }
}
