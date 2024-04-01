/*!
# Input Functionality

These functions enable menu/dialogue creation & interaction through
arrow-key selection and type-checking input boxes.
*/
use crate::{
    panic_menu,
    prelude::{error, invalid_input},
};
use dialoguer;
use std::{fmt::Display, io::Write, str::FromStr};

/**
Select an item using arrow keys from an array.

# Examples

```ignore
use albion_terminal_rpg::prelude::select;

let choice1: usize = select(&["1. Login", "2. Register", "3. Exit"], None);

let strings: Vec<String> = vec![String::from("Test"), String::from("No")];
let choice2: usize = select(&strings, Some("Select one of these."));
```
*/
pub fn select<T: Display>(options: &[T], optional_prompt: Option<&str>) -> usize {
    if let Some(prompt_text) = optional_prompt {
        println!("{prompt_text}");
    }

    dialoguer::Select::new()
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(0)
}

/**
Ask user for input with a prompt string.

# Examples

```ignore
use albion_terminal_rpg::prelude::prompt;

let answer: String = prompt("test"); // "test > {input here}"
```
*/
pub fn prompt(text: &str) -> String {
    print!("{text} > ");

    if let Err(message) = std::io::stdout().flush() {
        panic_menu!("Could not flush stdout: {}", message)
    }

    let mut input: String = String::new();

    if std::io::stdin().read_line(&mut input).is_err() {
        return prompt(text);
    }

    input.trim().to_string()
}

/**
Prompts user for input and attempts to
cast the string to a passed generic type.

# Examples

```ignore
use albion_terminal_rpg::prelude::{Result, input_generic};

let number: Result<usize> = input_generic("Number"); // "Number > {input here}"
let string: Result<String> = input_generic("Any String"); // "Any String > {input here}"
```
*/
pub fn generic_prompt<T>(text: &str) -> error::Result<T>
where
    T: FromStr,
{
    let input_string = prompt(text);
    let trimmed = input_string.trim();

    if let Ok(out) = trimmed.parse::<T>() {
        Ok(out)
    } else {
        invalid_input(Some(&input_string), None, false);
        Err(Box::new(error::Miscellaneous::InvalidInput(input_string)))
    }
}

/**
Ask a user for confirmation and wait for them type 'y' or 'n'.

- 'y', 'yes', and 'true' return true.
- 'n', 'no', and 'false' return false. (default for invalid input)

# Example

```ignore
use albion_terminal_rpg::prelude::confirm;

match confirm("Go to main menu?") {
    true => println!("Going to main menu."),
    false => println!("Cancelling."),
}
```
*/
pub fn confirm(prompt: &str) -> bool {
    let prompt = format!("{prompt} [y/n]");
    let input: error::Result<String> = generic_prompt(&prompt);

    if let Ok(input) = input {
        matches!(input.to_lowercase().as_str(), "yes" | "y" | "true")
    } else {
        false
    }
}

/**
Prompts for a password, hiding the text as it is typed.

# Example

```ignore
use albion_terminal_rpg::prelude::password;

let pass = password(false); // "Password: "
let confirm = password(true); // "Confirm:"
```
*/
pub fn password(confirm: bool) -> String {
    let string = if confirm { "Confirm Password > " } else { "Password > " };

    print!("{string}");
    std::io::stdout().flush().expect("Should flush stdout");

    match rpassword::read_password() {
        Ok(text) => text,
        Err(error) => panic_menu!("Failed to read password: {}", error),
    }
}
