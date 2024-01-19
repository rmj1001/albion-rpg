use crate::lib::tui::press_enter_to_continue;

fn response_msg<T>(message: T, pause: bool)
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

    response_msg(message, pause);
}

pub fn cancelling() {
    response_msg("Cancelling.", true);
}

pub fn success() {
    response_msg("Success!", true);
}

pub fn failure<T>(message: T)
where
    T: Into<String>,
{
    response_msg(format!("Failure: {}", message.into()), true);
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
