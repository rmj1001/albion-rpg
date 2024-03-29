/*!
# Error Handling

- Custom Result type
- Failure message generation for errors
- Custom panic screen for graceful program exiting
- Out of bounds function for less code duplication
*/
use crate::prelude::failure;
use std::fmt::{Debug, Display};
use thiserror::Error;

/**
Provide a simpler result format with dynamic error handling
# Usage

```
use albion_terminal_rpg::prelude::Result;

fn function() -> Result<()> {
    Ok(())
}
*/
pub type Result<T> = std::result::Result<T, Box<dyn CustomError>>;
pub trait CustomError
where
    Self: Display,
{
    fn failure(&self) {
        failure(&self.to_string());
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
#[derive(Debug, Clone, Error)]
pub enum ProfileError {
    #[error("Profile does not exist.")]
    DoesNotExist,

    #[error("Profile is corrupted.")]
    Corrupted,
}

impl CustomError for ProfileError {}

impl ProfileError {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

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
#[derive(Debug, Clone, Error)]
pub enum DataError {
    #[error("Failed to encode player file.")]
    Encode,

    #[error("Failed to decode player file.")]
    Decode,
}

impl CustomError for DataError {}

impl DataError {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

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
#[derive(Debug, Clone, Error)]
pub enum InventoryError {
    #[error("You do not have enough gold.")]
    NotEnoughGold,

    #[error("You already own this item.")]
    ItemOwned,

    #[error("You do not own this item.")]
    ItemNotOwned,

    #[error("You do not have enough xp.")]
    NotEnoughXP,

    #[error("You do not own enough {0}.")]
    NotEnoughItem(String),
}

impl CustomError for InventoryError {}

impl InventoryError {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

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
#[derive(Debug, Clone, Error)]
pub enum FileError {
    #[error("Failed to delete file.")]
    Delete,

    #[error("Failed to create file.")]
    Create,
}

impl CustomError for FileError {}

impl FileError {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

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
#[derive(Debug, Clone, Error)]
pub enum MiscError {
    #[error("Invalid input {0}")]
    InvalidInput(String),

    #[error("{0}")]
    Custom(&'static str),

    #[error("Invalid operator.")]
    InvalidOperator,
}

impl CustomError for MiscError {}

impl MiscError {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

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
pub fn out_of_bounds() {
    use crate::prelude::Logs;
    const MESSAGE: Option<&str> = Some("Dialoguer selected index out of bounds.");

    Logs::Failure.message(MESSAGE, None, true, true);
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
