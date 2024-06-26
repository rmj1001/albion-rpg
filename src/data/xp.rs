use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::prelude::{csv_table, error};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn table(&self) {
        fn entry(name: &str, xp: usize) -> String {
            format!("{},{},{}", name, xp, self::XP::get_level(xp))
        }

        csv_table(&vec![
            format!("Category,XP,Level"),
            entry("Combat", self.combat),
            entry("Fishing", self.fishing),
            entry("Cooking", self.cooking),
            entry("Woodcutting", self.woodcutting),
            entry("Mining", self.mining),
            entry("Smithing", self.smithing),
            entry("Thieving", self.thieving),
            entry("Player Total", self.total()),
        ]);
    }

    pub fn get_level(xp: usize) -> usize {
        (xp / 100) + 1
    }

    pub fn total(&self) -> usize {
        self.combat + self.fishing + self.cooking + self.woodcutting + self.mining + self.smithing + self.thieving
    }

    pub fn increment(&mut self, flag: XPType) {
        let more_xp = rand::thread_rng().gen_range(1..5);
        let xp = self.get(flag);

        *xp += more_xp;
    }

    pub fn add(&mut self, flag: XPType, amount: usize) -> error::Result<()> {
        let xp = self.get(flag);

        *xp += amount;
        Ok(())
    }

    pub fn subtract(&mut self, flag: XPType, amount: usize) -> error::Result<()> {
        let xp = self.get(flag);

        if *xp < amount {
            return Err(Box::new(error::Inventory::NotEnoughXP));
        }

        *xp -= amount;
        Ok(())
    }

    pub fn get(&mut self, flag: XPType) -> &mut usize {
        match flag {
            XPType::Combat => &mut self.combat,
            XPType::Fishing => &mut self.fishing,
            XPType::Cooking => &mut self.cooking,
            XPType::Woodcutting => &mut self.woodcutting,
            XPType::Mining => &mut self.mining,
            XPType::Smithing => &mut self.smithing,
            XPType::Thieving => &mut self.thieving,
        }
    }
}
