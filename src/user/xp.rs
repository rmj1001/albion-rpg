use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct XP {
    pub combat: u32,
    pub fishing: u32,
    pub cooking: u32,
    pub woodcutting: u32,
    pub mining: u32,
    pub smithing: u32,
    pub thieving: u32,
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
    pub fn level(xp: u32) -> u32 {
        (xp / 100) + 1
    }

    pub fn total_xp(&self) -> u32 {
        self.combat
            + self.fishing
            + self.cooking
            + self.woodcutting
            + self.mining
            + self.smithing
            + self.thieving
    }

    pub fn profile_level(&self) -> u32 {
        XP::level(self.total_xp())
    }

    pub fn arithmetic(&mut self, flag: XPType, operation: MathOp, amount: u32) -> Result<(), &str> {
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
            MathOp::Add => {
                *xp += amount;
                Ok(())
            }
            MathOp::Subtract => {
                if amount > *xp {
                    Err("The amount is greater than the total XP.")
                } else {
                    *xp -= amount;
                    Ok(())
                }
            }
            MathOp::Multiply => {
                *xp *= amount;
                Ok(())
            }
            MathOp::Divide => {
                *xp /= amount;
                Ok(())
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

    pub fn get(&self, flag: XPType) -> u32 {
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

pub enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}
