use std::str::FromStr;

use dialoguer::Confirm;

use crate::lib::tui::press_enter_to_continue;

/// Returns the index of the element in the vector selected.
pub fn selector(options: &[&str], optional_prompt: Option<&str>) -> usize {
    match optional_prompt {
        Some(prompt) => dialoguer::Select::new()
            .with_prompt(prompt)
            .items(options)
            .default(0)
            .interact()
            .unwrap_or(0),
        None => dialoguer::Select::new()
            .items(options)
            .default(0)
            .interact()
            .unwrap_or(0),
    }
}

pub fn select_from_vector(options: Vec<String>, optional_prompt: Option<&str>) -> usize {
    match optional_prompt {
        Some(prompt) => dialoguer::Select::new()
            .with_prompt(prompt)
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap_or(0),
        None => dialoguer::Select::new()
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap_or(0),
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

/// 'y' returns true, 'n' returns false.
pub fn confirm(prompt: &str) -> bool {
    loop {
        let input: Result<bool, dialoguer::Error> = Confirm::new().with_prompt(prompt).interact();

        match input {
            Ok(answer) => return answer,
            Err(_) => {
                invalid_input(None, None, true);
                continue;
            }
        }
    }
}

/// Prompts for a password, hiding the text as it is typed.
///
/// Parameters:
///
/// - confirm (bool) -> False: "Password: ", True: "Confirm Password:"
pub fn password(confirm: bool) -> String {
    let dialoguer_result = match confirm {
        true => dialoguer::Password::new()
            .with_prompt("Confirm Password")
            .interact(),
        false => dialoguer::Password::new()
            .with_prompt("Password")
            .interact(),
    };

    match dialoguer_result {
        Ok(text) => text,
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
/// Parameters:
///
/// - expected: The expected input
///
/// - pause: Ask the user to press enter to continue?
pub fn invalid_input(input: Option<&str>, expected: Option<&str>, pause: bool) {
    let mut output_string = String::new();

    match input {
        Some(text) => output_string.push_str(&format!("\nInvalid input '{}'.", text)),
        None => output_string.push_str("\nInvalid input."),
    }

    if let Some(text) = expected {
        output_string.push_str(&format!(" Expected '{}'.", text));
    }

    println!("{}", output_string);

    if pause {
        press_enter_to_continue();
    }
}
