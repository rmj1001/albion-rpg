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

/// Returns None if the input is not "yes", "no", "y", or "n".
/// If None then it also gives an error message.
pub fn yes_or_no(prompt: &str) -> Option<bool> {
    let input = prompt_input(&format!("{} (y/n)", prompt)).to_lowercase();

    match &input[..] {
        "y" => Some(true),
        "yes" => Some(true),
        "n" => Some(false),
        "no" => Some(false),
        invalid_input => {
            println!(
                "\nInvalid input '{}'. Expected 'yes' or 'no'.",
                invalid_input
            );
            press_enter_to_continue();
            None
        }
    }
}

pub fn password() -> String {
    dialoguer::Password::new()
        .with_prompt("Password")
        .interact()
        .unwrap()
}

pub fn out_of_bounds(optional_error: Option<&str>) {
    match optional_error {
        Some(error) => panic!("Dialogue selected index out of option's bounds: {}", error),
        None => panic!("Dialogue selected index out of option's bounds."),
    }
}
