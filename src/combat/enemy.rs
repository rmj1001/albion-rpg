use std::fmt::Display;

use crate::{
    data::{player::Player, xp::XP},
    prelude::random_num,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, PartialEq, EnumIter, Default, Copy)]
pub enum Enemies {
    // Human
    #[default]
    Human,
    Steve,

    // Animals
    Bear,
    DireWolf,
    GiantSpider,
    WhiteApe,
    Owlbear,
    Stag,
    Wyrm,

    // Monsters
    Centaur,
    DarkElf,
    Dragon,
    Giant,
    Goblin,
    Orc,
    Troll,
    Werewolf,
    Banshee,

    // Undead
    Ghost,
    Skeleton,
    Vampire,
    Zombie,
}

impl std::fmt::Display for Enemies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = match self {
            Enemies::DireWolf => "Dire Wolf".to_string(),
            Enemies::GiantSpider => "Giant Spider".to_string(),
            Enemies::WhiteApe => "White Ape".to_string(),
            Enemies::Owlbear => "Owl Bear".to_string(),
            Enemies::DarkElf => "Dark Elf".to_string(),

            // Should display one-word names as usual
            miscellaneous => format!("{miscellaneous:?}"),
        };

        write!(f, "{string}:")
    }
}

#[derive(Clone, Default)]
pub struct Data {
    pub flag: Enemies,
    pub name: String,
    pub hp: usize,
    pub damage: usize,
    pub rewards: Vec<Rewards>,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enemy: {}\nHP: {}\n", self.name, self.hp)
    }
}

impl Data {
    pub fn new(user_combat_xp: usize, user_hp: usize) -> Self {
        let user_level: usize = XP::get_level(user_combat_xp);
        let flag: Enemies = Self::enemy_type();
        let name: String = flag.to_string();

        Self {
            flag,
            name,
            hp: Self::hp(user_hp),
            damage: Self::damage(user_hp),
            rewards: Rewards::new(user_level),
        }
    }

    fn enemy_type() -> Enemies {
        let max = Enemies::iter().len();
        let number = random_num(0, max - 1);
        let option = Enemies::iter().get(number);

        if let Some(enemy) = option {
            enemy
        } else {
            crate::panic_menu!("Invalid enemy type chosen.")
        }
    }

    fn hp(player_hp: usize) -> usize {
        let deviation: usize = random_num(10, 30);
        let operation: usize = random_num(0, 1);

        match operation {
            0 => player_hp + deviation,
            1 => player_hp - deviation,
            _ => player_hp,
        }
    }

    fn damage(player_hp: usize) -> usize {
        let deviation: usize = random_num(1, 10);

        (player_hp / 10) + deviation
    }
}

// Rewards

#[derive(Debug, Clone)]
pub enum Rewards {
    XP(usize),
    Gold(usize),
    Potions(usize),
    Rubies(usize),
    MagicScrolls(usize),
    Bones(usize),
    DragonHides(usize),
    RunicTablets(usize),
}

impl std::fmt::Display for Rewards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = match self {
            Self::XP(xp) => format!("XP: {xp}"),
            Self::Gold(gold) => format!("Gold: {gold}"),
            Self::Potions(potions) => format!("Potions: {potions}"),
            Self::Rubies(rubies) => format!("Rubies: {rubies}"),
            Self::MagicScrolls(scrolls) => format!("Magic Scrolls: {scrolls}"),
            Self::Bones(bones) => format!("Bones: {bones}"),
            Self::DragonHides(hides) => format!("Dragon Hides: {hides}"),
            Self::RunicTablets(tablets) => format!("Runic Tablets: {tablets}"),
        };

        write!(f, "{string}")
    }
}

impl Default for Rewards {
    fn default() -> Self {
        Self::Potions(random_num(1, 3))
    }
}

impl Rewards {
    pub fn generate_quantity() -> usize {
        random_num(1, 3)
    }

    pub fn default_array() -> Vec<Self> {
        vec![Rewards::default(), Rewards::Bones(random_num(1, 3))]
    }

    pub fn new(player_level: usize) -> Vec<Self> {
        let mut rewards: Vec<Rewards> = Self::default_array();
        let xp_reward: usize = Self::xp(player_level);
        let mut gold_reward: usize = random_num(0, 10);

        // Generate XP

        if player_level > 10 {
            rewards.push(Rewards::MagicScrolls(Self::generate_quantity()));
            gold_reward += random_num(10, 20);
        }

        if player_level > 25 {
            rewards.push(Rewards::DragonHides(Self::generate_quantity()));
            gold_reward += random_num(20, 50);
        }

        if player_level > 50 {
            rewards.push(Rewards::Rubies(Self::generate_quantity()));
            gold_reward += random_num(50, 75);
        }

        if player_level > 100 {
            rewards.push(Rewards::RunicTablets(Self::generate_quantity()));
            gold_reward += random_num(75, 100);
        }

        rewards.push(Rewards::XP(xp_reward));
        rewards.push(Rewards::Gold(gold_reward));

        rewards
    }

    pub fn xp(player_level: usize) -> usize {
        let mut xp_reward: usize = random_num(0, 10);

        // Generate XP

        if player_level > 10 {
            xp_reward += random_num(10, 20);
        }

        if player_level > 25 {
            xp_reward += random_num(20, 50);
        }

        if player_level > 50 {
            xp_reward += random_num(50, 75);
        }

        if player_level > 100 {
            xp_reward += random_num(75, 100);
        }

        xp_reward
    }

    pub fn reward_to_player(player: &mut Player, rewards: Vec<Self>) {
        for reward in rewards {
            match reward {
                Rewards::Potions(quantity) => player.items.potions += quantity,
                Rewards::Bones(quantity) => player.items.bones += quantity,
                Rewards::Rubies(quantity) => player.items.rubies += quantity,
                Rewards::DragonHides(quantity) => player.items.dragon_hides += quantity,
                Rewards::MagicScrolls(quantity) => player.items.magic_scrolls += quantity,
                Rewards::RunicTablets(quantity) => player.items.runic_tablets += quantity,
                Rewards::Gold(gold) => player.bank.wallet += gold,
                Rewards::XP(xp) => player.xp.combat += xp,
            }
        }
    }
}
