use crate::prelude::failure;
use std::fmt::{Debug, Display};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn CustomError>>;
pub trait CustomError
where
    Self: Display,
{
    fn failure(&self) {
        failure(self.to_string());
    }

    fn display(&self) {
        println!("{}", self);
    }
}

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

#[derive(Debug, Clone, Error)]
pub enum IOError {
    #[error("Failed to delete file.")]
    DeleteFile,

    #[error("Failed to create file.")]
    CreateFile,
}

impl CustomError for IOError {}

impl IOError {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

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

pub fn check_debug_mode() -> bool {
    use std::env::args;
    use std::env::vars;

    let env_var: bool = vars().any(|(name, _)| name == "DEBUG");
    let args: bool = args().any(|arg| arg.to_lowercase() == "--debug");

    env_var || args
}

pub fn panic_menu_formatter<T: Display, U: Display, V: Display, W: Display>(
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
            Some(message) => failure(message.to_string()),
            None => failure(""),
        }
    }

    clearscr();
    std::process::exit(1);
}

#[macro_export]
macro_rules! panic_screen {
    () => {
        $crate::utils::error::panic_menu_formatter(file!(), line!(), column!(), None);
        std::process::exit(1);
    };

    ($fmt:expr) => ({
        $crate::utils::error::panic_menu_formatter(file!(), line!(), column!(), Some($fmt));
        std::process::exit(1);
    });

    ($fmt:expr, $($arg:tt)*) => ({
        $crate::utils::error::panic_menu_formatter(file!(), line!(), column!(), Some(format!($fmt, $($arg)*)));
        std::process::exit(1);
    });
}
