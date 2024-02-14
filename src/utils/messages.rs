use crate::utils::tui::press_enter_to_continue;

pub enum MessageLevel {
    Success,
    Failure,
    Warning,
    Cancelling,
    Note,
    Plain,
}

impl MessageLevel {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Success => Color::Green,
            Self::Failure => Color::Red,
            Self::Warning | Self::Cancelling => Color::Yellow,
            Self::Note => Color::Blue,
            Self::Plain => Color::None,
        }
    }
}
pub enum Color {
    Green,
    Red,
    Yellow,
    Blue,
    None,
}

impl Color {
    pub fn paint<T>(flag: Color, message: T) -> String
    where
        T: Into<String>,
    {
        let style = console::style(message.into());

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

pub fn build_response<T>(
    flag: MessageLevel,
    pause: bool,
    optional_description: Option<T>,
    optional_details: Option<T>,
    panic: bool,
) where
    T: Into<String>,
{
    let mut message: String = String::new();

    match flag {
        MessageLevel::Success => message.push_str("Success! "),
        MessageLevel::Failure => message.push_str("Failure! "),
        MessageLevel::Cancelling => message.push_str("Cancelling. "),
        MessageLevel::Warning => message.push_str("Warning! "),
        MessageLevel::Note => message.push_str("Note: "),
        MessageLevel::Plain => {}
    }

    if let Some(description) = optional_description {
        message.push_str(&description.into());
    }

    if let Some(details) = optional_details {
        message.push_str(&details.into());
    }

    let painted = Color::paint(flag.get_color(), message);

    if panic {
        panic!("\n{}", painted);
    } else {
        println!("\n{}", painted);
    }

    if pause {
        press_enter_to_continue();
    }
}

pub fn invalid_input(input: Option<&str>, expected: Option<&str>, pause: bool) {
    let mut description = String::new();

    if let Some(text) = input {
        description.push_str(&format!("Input: '{}'. ", text))
    }

    if let Some(text) = expected {
        description.push_str(&format!("Expected '{}'. ", text));
    }

    build_response(
        MessageLevel::Failure,
        pause,
        Some("Invalid input. "),
        Some(&description),
        false,
    );
}

pub fn warning<T>(subtext: Option<T>)
where
    T: Into<String>,
{
    build_response(MessageLevel::Warning, true, subtext, None, false);
}

pub fn cancelling() {
    build_response::<String>(MessageLevel::Cancelling, true, None, None, false);
}

pub fn cancel_msg<T>(subtext: T)
where
    T: Into<String>,
{
    build_response(MessageLevel::Cancelling, true, Some(subtext), None, false);
}

pub fn success() {
    build_response::<String>(MessageLevel::Success, true, None, None, false);
}

pub fn success_msg<T>(subtext: T)
where
    T: Into<String>,
{
    build_response(MessageLevel::Success, true, Some(subtext), None, false);
}

pub fn failure<T>(subtext: T)
where
    T: Into<String>,
{
    build_response(MessageLevel::Failure, true, Some(subtext), None, false);
}

pub fn note<T>(description: T, pause: bool)
where
    T: Into<String>,
{
    build_response(MessageLevel::Note, pause, Some(description), None, false);
}

pub fn plain_respond<T>(description: T, pause: bool)
where
    T: Into<String>,
{
    build_response(MessageLevel::Plain, pause, Some(description), None, false);
}

/// Standard panic message for dialogue selector
pub fn out_of_bounds() {
    build_response(
        MessageLevel::Failure,
        false,
        Some("Dialoguer selected index out of bounds."),
        None,
        true,
    );
}
