use crate::lib::tui::press_enter_to_continue;

pub fn response_msg<T>(message: T, pause: bool)
where
    T: Into<String>,
{
    println!("\n{}", message.into());

    if pause {
        press_enter_to_continue();
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
    let mut message = String::new();

    match input {
        Some(text) => message.push_str(&format!("\nInvalid input '{}'.", text)),
        None => message.push_str("\nInvalid input."),
    }

    if let Some(text) = expected {
        message.push_str(&format!(" Expected '{}'.", text));
    }

    response_msg(console::style(message).red().bright().to_string(), pause);
}

pub fn warn<T>(optional_message: Option<T>)
where
    T: Into<String>,
{
    let mut message = "Warning: ".to_string();

    if let Some(added_text) = optional_message {
        message.push_str(&added_text.into());
    }

    let painted: String = console::style(message).yellow().to_string();
    response_msg(painted, true);
}

pub fn cancelling() {
    let message: &str = "Cancelling.";
    let painted: String = console::style(message).yellow().to_string();
    response_msg(painted, true);
}

pub fn custom_cancel<T>(added_message: T)
where
    T: Into<String>,
{
    let message: String = format!("Cancelling. {}", added_message.into());
    let painted: String = console::style(message).yellow().bright().to_string();
    response_msg(painted, true);
}

pub fn success() {
    let message: &str = "Success!";
    let painted: String = console::style(message).green().bright().to_string();
    response_msg(painted, true);
}

pub fn custom_success<T>(added_message: T)
where
    T: Into<String>,
{
    let message: String = format!("Success! {}", added_message.into());
    let painted: String = console::style(message).green().bright().to_string();
    response_msg(painted, true);
}

pub fn failure<T>(added_message: T)
where
    T: Into<String>,
{
    let message: String = format!("Failure: {}", added_message.into());
    let painted: String = console::style(message).red().bright().to_string();
    response_msg(painted, true);
}

/// Standard panic message for dialogue selector
pub fn out_of_bounds<T>(optional_error: Option<T>)
where
    T: Into<String>,
{
    match optional_error {
        Some(error) => panic!(
            "\nDialogue selected index out of option's bounds: {}",
            error.into()
        ),
        None => panic!("\nDialogue selected index out of option's bounds."),
    }
}
