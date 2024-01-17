use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::lib::{
    math::Operation,
    tui::{press_enter_to_continue, print_table},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct XP {
    pub combat: usize,
    pub fishing: usize,
    pub cooking: usize,
    pub woodcutting: usize,
    pub mining: usize,
    pub smithing: usize,
    pub thieving: usize,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
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
    pub fn print_table(&self) {
        print_table(vec![
            "Category,XP,Level".to_string(),
            format!("Combat,{},{}", self.combat, self::XP::level(self.combat)),
            format!("Fishing,{},{}", self.fishing, self::XP::level(self.fishing)),
            format!("Cooking,{},{}", self.cooking, self::XP::level(self.cooking)),
            format!(
                "Woodcutting,{},{}",
                self.woodcutting,
                self::XP::level(self.woodcutting)
            ),
            format!("Mining,{},{}", self.mining, self::XP::level(self.mining)),
            format!(
                "Smithing,{},{}",
                self.smithing,
                self::XP::level(self.smithing)
            ),
            format!(
                "Thieving,{},{}",
                self.thieving,
                self::XP::level(self.thieving)
            ),
            format!("Profile Total,{},{}", self.total_xp(), self.profile_level()),
        ])
    }

    pub fn level(xp: usize) -> usize {
        (xp / 100) + 1
    }

    pub fn total_xp(&self) -> usize {
        self.combat
            + self.fishing
            + self.cooking
            + self.woodcutting
            + self.mining
            + self.smithing
            + self.thieving
    }

    pub fn profile_level(&self) -> usize {
        XP::level(self.total_xp())
    }

    pub fn arithmetic(&mut self, flag: XPType, operation: Operation<usize>) -> Result<(), &str> {
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
                    Err("The amount is greater than the total XP.")
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
                println!("\nCancelling.");
                press_enter_to_continue();
                Ok(())
            }
            Operation::Invalid => {
                println!("\nOperation failed: Invalid Operator");
                press_enter_to_continue();
                Err("")
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
