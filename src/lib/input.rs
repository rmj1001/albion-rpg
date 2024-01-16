use std::{io::Write, str::FromStr};

use dialoguer::Confirm;

use crate::lib::tui::press_enter_to_continue;

pub fn select_from_str_array(options: &[&str], optional_prompt: Option<&str>) -> usize {
    if let Some(prompt_text) = optional_prompt {
        println!("{prompt_text}");
    }

    dialoguer::Select::new()
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(0)
}

pub fn select_from_vector(options: Vec<String>, optional_prompt: Option<&str>) -> usize {
    if let Some(prompt_text) = optional_prompt {
        println!("{prompt_text}");
    }

    dialoguer::Select::new()
        .items(&options[..])
        .default(0)
        .interact()
        .unwrap_or(0)
}

/// NOTE: Don't use this unless you're using a custom prompt end character
pub fn prompt(text: &str) -> String {
    print!("{text} ");

    std::io::stdout().flush().expect("Could not flush stdout");

    let mut input: String = String::new();

    if std::io::stdin().read_line(&mut input).is_err() {
        return prompt(text);
    }

    input.trim().to_string()
}

/// Example: prompt_colon("test"); -> test: {input here}
pub fn prompt_colon(text: &str) -> String {
    prompt(&format!("{text}:"))
}

/// Example: prompt_arrow("test"); -> test > {input here}
pub fn prompt_arrow(text: &str) -> String {
    prompt(&format!("{text} >"))
}

/// Attempts to cast the string to a generic type
pub fn input_generic<T>(text: &str) -> Result<T, &str>
where
    T: FromStr,
{
    let input_string = prompt(text);
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
