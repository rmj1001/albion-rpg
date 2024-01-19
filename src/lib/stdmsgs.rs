use std::fmt::Display;

use crate::lib::tui::press_enter_to_continue;

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

pub fn cancelling() {
    println!("\nCancelling.");
    press_enter_to_continue();
}

pub fn success() {
    println!("\nSuccess!");
    press_enter_to_continue();
}

pub fn error<T>(error: T)
where
    T: Display,
{
    eprintln!("\nError: {}", error);
    press_enter_to_continue();
}

/// Standard panic message for dialogue selector
pub fn out_of_bounds(optional_error: Option<&str>) {
    match optional_error {
        Some(error) => panic!("Dialogue selected index out of option's bounds: {}", error),
        None => panic!("Dialogue selected index out of option's bounds."),
    }
}
