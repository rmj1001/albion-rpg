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
