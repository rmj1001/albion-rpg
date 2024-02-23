use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{math::Operation, messages::*, tui::table_from_csv},
    MiscError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XP {
    pub combat: usize,
    pub fishing: usize,
    pub cooking: usize,
    pub woodcutting: usize,
    pub mining: usize,
    pub smithing: usize,
    pub thieving: usize,
}

#[derive(Clone, Copy)]
pub enum XPType {
    Combat,
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
    Thieving,
}

impl XP {
    pub fn new() -> Self {
        Self {
            combat: 0,
            fishing: 0,
            cooking: 0,
            woodcutting: 0,
            mining: 0,
            smithing: 0,
            thieving: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(name: &str, xp: usize) -> String {
            format!("{},{},{}", name, xp, self::XP::get_level(xp))
        }

        table_from_csv(vec![
            format!("Category,XP,Level"),
            entry("Combat", self.combat),
            entry("Fishing", self.fishing),
            entry("Cooking", self.cooking),
            entry("Woodcutting", self.woodcutting),
            entry("Mining", self.mining),
            entry("Smithing", self.smithing),
            entry("Thieving", self.thieving),
            entry("Player Total", self.total()),
        ])
    }

    pub fn get_level(xp: usize) -> usize {
        (xp / 100) + 1
    }

    pub fn total(&self) -> usize {
        self.combat + self.fishing + self.cooking + self.woodcutting + self.mining + self.smithing + self.thieving
    }

    pub fn arithmetic(&mut self, flag: XPType, operation: Operation<usize>) -> crate::Result<()> {
        let xp = match flag {
            XPType::Combat => &mut self.combat,
            XPType::Fishing => &mut self.fishing,
            XPType::Cooking => &mut self.cooking,
            XPType::Woodcutting => &mut self.woodcutting,
            XPType::Mining => &mut self.mining,
            XPType::Smithing => &mut self.smithing,
            XPType::Thieving => &mut self.thieving,
        };

        match operation {
            Operation::Add(amount) => {
                *xp += amount;
                Ok(())
            }
            Operation::Subtract(amount) => {
                if amount > *xp {
                    Err(MiscError::Custom("The amount is greater than the total XP.").boxed())
                } else {
                    *xp -= amount;
                    Ok(())
                }
            }
            Operation::Multiply(amount) => {
                *xp *= amount;
                Ok(())
            }
            Operation::Divide(amount) => {
                *xp /= amount;
                Ok(())
            }
            Operation::Cancel => {
                cancelling();
                Ok(())
            }
            Operation::Invalid => {
                failure("Invalid Operator");
                Err(MiscError::InvalidOperator.boxed())
            }
        }
    }

    pub fn increment(&mut self, flag: XPType) {
        let more_xp = rand::thread_rng().gen_range(1..5);

        match flag {
            XPType::Combat => self.combat += more_xp,
            XPType::Fishing => self.fishing += more_xp,
            XPType::Cooking => self.cooking += more_xp,
            XPType::Woodcutting => self.woodcutting += more_xp,
            XPType::Mining => self.mining += more_xp,
            XPType::Smithing => self.smithing += more_xp,
            XPType::Thieving => self.thieving += more_xp,
        }
    }

    pub fn get(&self, flag: XPType) -> usize {
        match flag {
            XPType::Combat => self.combat,
            XPType::Fishing => self.fishing,
            XPType::Cooking => self.cooking,
            XPType::Woodcutting => self.woodcutting,
            XPType::Mining => self.mining,
            XPType::Smithing => self.smithing,
            XPType::Thieving => self.thieving,
        }
    }
}
