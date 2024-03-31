/*!
# Error Handling

- Custom Result type
- Failure message generation for errors
- Custom panic screen for graceful program exiting
- Out of bounds function for less code duplication
*/
use crate::utils::messages::Logs;
use std::error::Error;
use std::fmt::{Debug, Display};

/**
Provide a simpler result format with dynamic error handling
# Usage

```
use albion_terminal_rpg::prelude::Result;

fn function() -> Result<()> {
    Ok(())
}
*/
pub type Result<T> = std::result::Result<T, Box<dyn Printer>>;

pub trait Printer
where
    Self: Display,
{
    fn print(&self, pause: bool) {
        println!("{self}");

        if pause {
            crate::utils::tui::pause();
        }
    }
}

/**
Profile corruption/absense errors

# Examples

```panics
use albion_terminal_rpg::prelude::{ProfileError, CustomError};

fn main() -> Result<()> {
    ProfileError::DoesNotExist.failure();

    Err(ProfileError::Corrupted);
}
```
*/
#[derive(Debug, Clone)]
pub enum ProfileError {
    DoesNotExist,
    Corrupted,
}

impl Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: &str = match self {
            Self::DoesNotExist => "Profile does not exist.",
            Self::Corrupted => "Profile is corrupted.",
        };

        write!(f, "{}", Logs::Failure.paint(string))
    }
}

impl Printer for ProfileError {}
impl Error for ProfileError {}

/**
Data Serialization/Deserialization Errors

# Examples

```panics
use albion_terminal_rpg::prelude::{DataError, CustomError};

fn main() -> Result<()> {
    DataError::Encode.failure();

    Err(DataError::Decode);
}
```
*/
#[derive(Debug, Clone)]
pub enum DataError {
    Encode,
    Decode,
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: &str = match self {
            Self::Encode => "Failed to encode player file.",
            Self::Decode => "Failed to decode player file.",
        };

        write!(f, "{}", Logs::Failure.paint(string))
    }
}

impl Printer for DataError {}
impl Error for DataError {}

/**
Inventory management errors

# Examples

```panics
use albion_terminal_rpg::prelude::{InventoryError, CustomError};

fn main() -> Result<()> {
    InventoryError::ItemOwned.failure();

    Err(InventoryError::NotEnoughItem("Bones".to_string()));
}
```
*/
#[derive(Debug, Clone)]
pub enum InventoryError {
    NotEnoughGold,
    ItemOwned,
    ItemNotOwned,
    NotEnoughXP,
    NotEnoughItem(String),
    ItemNotExist,
    TransactionFailed,
}

impl Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = match self {
            Self::ItemNotExist => "That item does not exist.".to_string(),
            Self::ItemNotOwned => "You do not own that item.".to_string(),
            Self::ItemOwned => "You already own this item".to_string(),
            Self::NotEnoughGold => "You do not have enough gold.".to_string(),
            Self::NotEnoughItem(item) => format!("You do not own enough {item}."),
            Self::NotEnoughXP => "You do not have enough xp.".to_string(),
            Self::TransactionFailed => "Transaction failed.".to_string(),
        };

        write!(f, "{}", Logs::Failure.paint(string))
    }
}

impl Printer for InventoryError {}
impl Error for InventoryError {}

/**
File management Errors

# Examples

```panics
use albion_terminal_rpg::prelude::{FileError, CustomError};

fn main() -> Result<()> {
    FileError::Delete.failure();

    Err(FileError::Create);
}
```
*/
#[derive(Debug, Clone)]
pub enum FileError {
    Delete,
    Create,
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Delete => "Failed to delete file.",
            Self::Create => "Failed to create file.",
        };

        write!(f, "{}", Logs::Failure.paint(string))
    }
}

impl Printer for FileError {}
impl Error for FileError {}

/**
Miscellaneous Errors

# Examples

```panics
use albion_terminal_rpg::prelude::{MiscError, CustomError};

fn main() -> Result<()> {
    MiscError::InvalidInput("text").failure();

    Err(MiscError::Custom("custom error message"));
}
```
*/
#[derive(Debug, Clone)]
pub enum MiscError {
    InvalidInput(String),
    Custom(&'static str),
    InvalidOperator,
}

impl Display for MiscError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = match self {
            Self::InvalidInput(input) => format!("Invalid input {input}."),
            Self::Custom(error) => error.to_string(),
            Self::InvalidOperator => "Invalid operator.".to_string(),
        };

        write!(f, "{}", Logs::Failure.paint(string))
    }
}

impl Printer for MiscError {}
impl Error for MiscError {}

/**
Check for debugging environment variable or flag

# Usage

```
use albion_terminal_rpg::prelude::{check_debug_mode};

let debug = check_debug_mode();
println!("{}", debug)
```

# Command Line

```bash
// Environment Variable
DEBUG=1 albionrpg

// Debug flag
albionrpg --debug
```
*/
pub fn check_debug_mode() -> bool {
    use std::env::args;
    use std::env::vars;

    let env_var: bool = vars().any(|(name, _)| name == "DEBUG");
    let args: bool = args().any(|arg| arg.to_lowercase() == "--debug");

    env_var || args
}

/**
Out of bounds failure message for vector item selection

# Usage

```panics
use albion_terminal_rpg::prelude::{out_of_bounds};

out_of_bounds();
```
*/
pub fn unreachable() {
    const MESSAGE: &str = "Dialoguer selected index out of bounds.";
    unreachable!("{}", Logs::Failure.paint(MESSAGE));
}

pub mod macros {
    use crate::prelude::{check_debug_mode, failure};
    use std::fmt::Display;

    /**
    Build a custom panic screen for graceful exiting in fatal errors.

    # Usage

    ```panics
    use albion_terminal_rpg::prelude::{macros::panic_builder};

    panic_builder(file!(), line!(), column!(), Some("error message here"));
    ```
    */
    pub fn panic_builder<T: Display, U: Display, V: Display, W: Display>(
        file: T,
        line: U,
        column: V,
        message: Option<W>,
    ) {
        use crate::utils::terminal::clearscr;
        use crate::utils::tui::{page_header, pause, Instructions};

        page_header("Error", Instructions::None);

        if check_debug_mode() {
            println!("File: {}", file.to_string().trim());
            println!();
            println!("Line: {}", line.to_string().trim());
            println!();
            println!("Column: {}", column.to_string().trim());

            if let Some(message) = message {
                println!();
                println!("Message: {}", message.to_string().trim());
            }

            println!();
            pause();
        } else {
            match message {
                Some(message) => failure(&message.to_string()),
                None => failure(""),
            }
        }

        clearscr();
        std::process::exit(1);
    }

    /**
    Custom panic macro for graceful shutdown upon fatal error

    # Usage

    ```panics
    use albion_terminal_rpg::panic_screen;

    panic_menu!("error message here")
    ```
    */
    #[macro_export]
    macro_rules! panic_menu {
        () => {
            $crate::utils::error::panic_builder(file!(), line!(), column!(), None);
            std::process::exit(1);
        };

        ($fmt:expr) => ({
            $crate::utils::error::macros::panic_builder(file!(), line!(), column!(), Some($fmt));
            std::process::exit(1);
        });

        ($fmt:expr, $($arg:tt)*) => ({
            $crate::utils::error::macros::panic_builder(file!(), line!(), column!(), Some(format!($fmt, $($arg)*)));
            std::process::exit(1);
        });
    }
}
