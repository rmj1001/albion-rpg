use super::messages::failure;
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
