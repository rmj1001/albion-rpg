/// Returns the index of the element in the vector selected.
pub fn selector(options: &[&str], default_index: usize, optional_prompt: Option<&str>) -> usize {
    match optional_prompt {
        Some(prompt) => {
            if prompt.is_empty() {
                dialoguer::Select::new()
                    .items(options)
                    .default(0)
                    .interact()
                    .unwrap_or(default_index)
            } else {
                dialoguer::Select::new()
                    .with_prompt(prompt)
                    .items(options)
                    .default(0)
                    .interact()
                    .unwrap_or(default_index)
            }
        }
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
        Some(prompt) => {
            if prompt.is_empty() {
                dialoguer::Select::new()
                    .items(&options[..])
                    .default(0)
                    .interact()
                    .unwrap_or(default_index)
            } else {
                dialoguer::Select::new()
                    .with_prompt(prompt)
                    .items(&options[..])
                    .default(0)
                    .interact()
                    .unwrap_or(default_index)
            }
        }
        None => dialoguer::Select::new()
            .with_prompt(
                "Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it",
            )
            .items(&options[..])
            .default(0)
            .interact()
            .unwrap_or(default_index),
    }
}

pub fn prompt_input(prompt: &str) -> String {
    dialoguer::Input::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

pub fn password() -> String {
    dialoguer::Password::new()
        .with_prompt("Password")
        .interact()
        .unwrap()
}
