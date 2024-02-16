use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MundaneInventory {
    pub bait: usize,
    pub seeds: usize,
    pub furs: usize,
    pub fish: usize,
    pub food: usize,
    pub wood: usize,
    pub ore: usize,
    pub ingots: usize,
    pub potions: usize,
    pub rubies: usize,
    pub magic_scrolls: usize,
    pub bones: usize,
    pub dragon_hides: usize,
    pub runic_tablets: usize,
}

impl MundaneInventory {
    pub fn new() -> MundaneInventory {
        MundaneInventory {
            bait: 0,
            seeds: 0,
            furs: 0,
            fish: 0,
            food: 0,
            wood: 0,
            ore: 0,
            ingots: 0,
            potions: 0,
            rubies: 0,
            magic_scrolls: 0,
            bones: 0,
            dragon_hides: 0,
            runic_tablets: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
