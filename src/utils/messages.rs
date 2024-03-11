use crate::prelude::pause;
use std::fmt::Display;

pub enum MessageLevel {
    Success,
    Failure,
    Warning,
    Cancelling,
}

impl MessageLevel {
    /// Success is Green, Failure is Red, Warnings/Cancelling is Yellow, Notes are Blue.
    pub fn get_color(&self) -> Color {
        match self {
            Self::Success => Color::Green,
            Self::Failure => Color::Red,
            Self::Warning | Self::Cancelling => Color::Yellow,
        }
    }
}

/// The allowed colors are Green, Red, Yellow, and Blue.
pub enum Color {
    Green,
    Red,
    Yellow,
    Blue,
    None,
}

impl Color {
    pub fn paint<T: Display>(flag: Color, message: T) -> String {
        let style = console::style(message.to_string());

        let painted = match flag {
            Color::Green => style.green(),
            Color::Red => style.red(),
            Color::Yellow => style.yellow(),
            Color::Blue => style.blue(),
            Color::None => style.white(),
        };

        painted.bright().to_string()
    }
}

/// Create a painted response string with a press_enter_to_continue
pub fn response<T: ToString>(
    flag: MessageLevel,
    use_pause: bool,
    optional_description: Option<T>,
    optional_details: Option<T>,
    panic: bool,
) {
    let mut message: String = String::new();

    match flag {
        MessageLevel::Success => message.push_str("Success! "),
        MessageLevel::Failure => message.push_str("Failure! "),
        MessageLevel::Cancelling => message.push_str("Cancelling. "),
        MessageLevel::Warning => message.push_str("Warning! "),
    }

    if let Some(description) = optional_description {
        message.push_str(&description.to_string());
    }

    if let Some(details) = optional_details {
        message.push_str(&details.to_string());
    }

    let painted = Color::paint(flag.get_color(), message);

    if panic {
        panic!("\n{}", painted);
    } else {
        println!("\n{}", painted);
    }

    if use_pause {
        pause();
    }
}

/// Use this only if accepting typed input
pub fn invalid_input(input: Option<&str>, expected: Option<&str>, pause: bool) {
    let mut description = String::new();

    if let Some(text) = input {
        description.push_str(&format!("Input: '{}'. ", text))
    }

    if let Some(text) = expected {
        description.push_str(&format!("Expected '{}'. ", text));
    }

    response(
        MessageLevel::Failure,
        pause,
        Some("Invalid input. "),
        Some(&description),
        false,
    );
}

/// Yellow text. "Warning: " prefix
pub fn warning(subtext: Option<&str>) {
    response(MessageLevel::Warning, true, subtext, None, false);
}

pub fn cancel(subtext: Option<&str>) {
    response(MessageLevel::Cancelling, true, subtext, None, false);
}

/// Green text. "Success!" prefix.
pub fn success(subtext: Option<&str>) {
    response(MessageLevel::Success, true, subtext, None, false);
}

/// Red text. "Failure!" prefix. Custom Suffix.
pub fn failure(subtext: &str) {
    response(MessageLevel::Failure, true, Some(subtext), None, false);
}
