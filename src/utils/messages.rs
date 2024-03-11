/*!
# Messages

Standardized response messages with coloring!
*/
use crate::prelude::pause;
use std::fmt::Display;

/**
Standard levels for logging and coloring.
*/
pub enum Logs {
    Success,
    Failure,
    Warning,
    Cancelling,
}

impl Logs {
    /**
    Paint a string with the color associated with the chosen log level.

    Colors:
    - Success: Green
    - Failure: Red
    - Warning/Cancelling: Yellow

    # Examples

    ```
    use albion_terminal_rpg::prelude::Logs;

    let message1 = Logs::Success.paint("This was a success!");
    let message2 = Logs::Failure.paint("This was a failure :(");
    ```
    */
    pub fn paint<T: Display>(&self, message: T) -> String {
        let style = console::style(message.to_string());

        let painted = match self {
            Self::Success => style.green(),
            Self::Failure => style.red(),
            Self::Warning | Self::Cancelling => style.yellow(),
        };

        painted.bright().to_string()
    }

    /**
    Generate a response with optional pausing, panicing, etc.

    # Examples

    ```
    use albion_terminal_rpg::prelude::Logs;

    // Success
    Logs::Success.message(Some("This was a success!"), None, true, false);

    // Failure
    Logs::Failure.message(Some("This was a failure :("), None, true, false);
    ```
    */
    pub fn message<T: Display>(&self, description: Option<T>, extra_details: Option<T>, use_pause: bool, panic: bool) {
        let mut message: String = String::new();

        match self {
            Self::Success => message.push_str("Success! "),
            Self::Failure => message.push_str("Failure! "),
            Self::Cancelling => message.push_str("Cancelling. "),
            Self::Warning => message.push_str("Warning! "),
        }

        if let Some(description) = description {
            message.push_str(&description.to_string());
        }

        if let Some(extra_details) = extra_details {
            message.push_str(&extra_details.to_string());
        }

        let painted = self.paint(message);

        if panic {
            panic!("\n{}", painted);
        } else {
            println!("\n{}", painted);
        }

        if use_pause {
            pause();
        }
    }
}

/**
Common failure message for invalidly typed input.
Mostly used for either menu codes or type casting.

# Examples

```
use albion_terminal_rpg::prelude::invalid_input;

// Show given input but not expected input & pause
invalid_input(Some("nrgorg"), None, true);

// shows given input and expected input & pause
invalid_input(Some("-1"), Some("Positive Integer"), true);
```
*/
pub fn invalid_input(input: Option<&str>, expected: Option<&str>, pause: bool) {
    let mut description = String::new();

    if let Some(text) = input {
        description.push_str(&format!("Input: '{}'. ", text))
    }

    if let Some(text) = expected {
        description.push_str(&format!("Expected '{}'. ", text));
    }

    Logs::Failure.message(Some("Invalid Input."), Some(&description), pause, false);
}

/**
Yellow-colored "Warning!" message with optional subtext

# Examples

```
use albion_terminal_rpg::prelude::warning;

// Output: "Warning!"
warning(None);

// Output: "Warning! You're about to mess up!"
warning(Some("You're about to mess up!"));
```
*/
pub fn warning(subtext: Option<&str>) {
    Logs::Warning.message(subtext, None, true, false);
}

/**
Yellow-colored "Cancelling." message with optional subtext

# Examples

```
use albion_terminal_rpg::prelude::cancel;

// Output: "Cancelling."
cancel(None);

// Output: "Cancelling. Going back to main menu."
cancel(Some("Going back to main menu."));
```
*/
pub fn cancel(subtext: Option<&str>) {
    Logs::Cancelling.message(subtext, None, true, false);
}

/**
Green-colored "Success!" message with optional subtext

# Examples

```
use albion_terminal_rpg::prelude::success;

// Output: "Success!"
success(None);

// Output: "Success! Going back to main menu."
success(Some("Going back to main menu."));
```
*/
pub fn success(subtext: Option<&str>) {
    Logs::Success.message(subtext, None, true, false);
}

/**
Green-colored "Failure!" message with **REQUIRED** subtext.

# Example

```
use albion_terminal_rpg::prelude::failure;

// Output: "Failure! You need to select a different item."
failure("You need to select a different item.");
```
*/
pub fn failure(subtext: &str) {
    Logs::Failure.message(Some(subtext), None, true, false);
}
