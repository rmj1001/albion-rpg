use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Armor {
    pub name: String,
    pub price: u32,
    pub owns: bool,
    pub defense: u16,
    pub durability: i16,
    pub default_durability: i16,
}

impl Armor {
    pub fn decrease_durability(&mut self) {
        let random_damage = rand::thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability <= 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArmorInventory {
    pub leather: Armor,
    pub bronze: Armor,
    pub iron: Armor,
    pub steel: Armor,
    pub dragonhide: Armor,
    pub mystic: Armor,
}
