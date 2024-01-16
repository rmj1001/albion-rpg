use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Weapon {
    pub name: String,
    pub price: usize,
    pub owns: bool,
    pub damage: usize,
    pub durability: usize,
    pub default_durability: usize,
}

impl Weapon {
    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);
        self.durability -= random_damage;

        if self.durability == 0 {
            self.owns = false;
            self.durability = self.default_durability
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeaponsInventory {
    pub wooden_sword: Weapon,
    pub bronze_sword: Weapon,
    pub iron_sword: Weapon,
    pub steel_sword: Weapon,
    pub mystic_sword: Weapon,
    pub wizard_staff: Weapon,
}
