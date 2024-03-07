use crate::{
    data::{player::Player, xp::XP},
    utils::math::{self, random_num},
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, PartialEq, EnumIter)]
pub enum Enemy {
    // Human
    Human,
    Steve,

    // Animals
    Bear,
    DireWolf,
    GiantSpider,
    WhiteApe,
    Owlbear,
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

#[derive(Clone)]
pub struct EnemyData {
    pub kind_type: Enemy,
    pub kind: &'static str,
    pub hp: usize,
    pub damage: usize,
    pub xp: usize,
    pub gold: usize,
    pub rewards: Vec<Rewards>,
}

impl EnemyData {
    pub fn new(user_combat_xp: usize, user_hp: usize) -> Self {
        let user_level: usize = XP::get_level(user_combat_xp);
        let kind_type: Enemy = pick_enemy();
        let kind_string: &str = EnemyData::name(kind_type.clone());

        Self {
            kind_type,
            kind: kind_string,
            hp: calculate_hp(user_hp),
            damage: calculate_damage(user_hp),
            xp: linear_xp_gold(user_level),
            gold: linear_xp_gold(user_level),
            rewards: generate_rewards(user_level),
        }
    }

    pub fn name(kind: Enemy) -> &'static str {
        match kind {
            // Human
            Enemy::Human => "Human",
            Enemy::Steve => "Steve",

            // Animal
            Enemy::Bear => "Bear",
            Enemy::DireWolf => "Dire Wolf",
            Enemy::GiantSpider => "Giant Spider",
            Enemy::WhiteApe => "White Ape",
            Enemy::Owlbear => "Owlbear",
            Enemy::Wyrm => "Wyrm",

            // Undead
            Enemy::Banshee => "Banshee",
            Enemy::Ghost => "Ghost",
            Enemy::Zombie => "Zombie",
            Enemy::Vampire => "Vampire",
            Enemy::Skeleton => "Skeleton",

            // Monster
            Enemy::Centaur => "Centaur",
            Enemy::Orc => "Orc",
            Enemy::Giant => "Giant",
            Enemy::Troll => "Troll",
            Enemy::Dragon => "Dragon",
            Enemy::Goblin => "Goblin",
            Enemy::DarkElf => "Dark Elf",
            Enemy::Werewolf => "Werewolf",
        }
    }
}

fn pick_enemy() -> Enemy {
    let max = Enemy::iter().len();
    let number = math::random_num(0, max - 1);

    Enemy::iter().get(number).expect("Should return a valid enemy")
}

// Strength

fn calculate_hp(player_hp: usize) -> usize {
    let deviation: usize = random_num(10, 30);
    let operation: usize = random_num(0, 1);

    match operation {
        0 => player_hp + deviation,
        1 => player_hp - deviation,
        _ => player_hp,
    }
}

fn calculate_damage(player_hp: usize) -> usize {
    let deviation: usize = random_num(1, 10);

    (player_hp / 10) + deviation
}

// Rewards

#[derive(Debug, Clone)]
pub enum Rewards {
    Potions(usize),
    Rubies(usize),
    MagicScrolls(usize),
    Bones(usize),
    DragonHides(usize),
    RunicTablets(usize),
    Invalid,
}

pub fn linear_xp_gold(player_level: usize) -> usize {
    let mut working_xp: usize = 0;

    if player_level > 0 {
        working_xp += random_num(0, 10);
    }

    if player_level > 10 {
        working_xp += random_num(10, 20);
    }

    if player_level > 25 {
        working_xp += random_num(20, 50);
    }

    if player_level > 50 {
        working_xp += random_num(50, 75);
    }

    if player_level > 100 {
        working_xp += random_num(75, 100)
    }

    working_xp
}

pub fn generate_rewards(player_level: usize) -> Vec<Rewards> {
    let mut rewards: Vec<Rewards> = vec![Rewards::Potions(random_num(1, 3)), Rewards::Bones(random_num(1, 3))];

    if player_level > 10 {
        rewards.push(Rewards::MagicScrolls(random_num(1, 3)));
    }

    if player_level > 25 {
        rewards.push(Rewards::DragonHides(random_num(1, 3)));
    }

    if player_level > 50 {
        rewards.push(Rewards::Rubies(random_num(1, 3)));
    }

    if player_level > 100 {
        rewards.push(Rewards::RunicTablets(random_num(1, 3)));
    }

    rewards
}

pub fn add_rewards_to_user(player: &mut Player, rewards: Vec<Rewards>) {
    rewards.iter().for_each(|reward| match reward {
        Rewards::Potions(quantity) => player.items.potions += quantity,
        Rewards::Bones(quantity) => player.items.bones += quantity,
        Rewards::Rubies(quantity) => player.items.rubies += quantity,
        Rewards::DragonHides(quantity) => player.items.dragon_hides += quantity,
        Rewards::MagicScrolls(quantity) => player.items.magic_scrolls += quantity,
        Rewards::RunicTablets(quantity) => player.items.runic_tablets += quantity,
        Rewards::Invalid => {}
    });
}

pub mod tests {
    #[test]
    pub fn invalid_enemies() {
        use crate::combat::enemy::{Enemy, EnemyData};
        use crate::data::player::Player;
        use crate::utils::crypt;

        let test_player = Player::new("test", &crypt::generate_hash("test".to_string()), false);

        let num_enemies: usize = 500;
        let mut enemies: Vec<EnemyData> = vec![];

        let mut index: usize = 0;

        while index < num_enemies {
            enemies.push(EnemyData::new(test_player.xp.combat, test_player.health.hp));
            index += 1
        }

        let invalids: Vec<EnemyData> = enemies.to_vec();
        let mut types: Vec<Enemy> = vec![];

        invalids
            .iter()
            .for_each(|enemy: &EnemyData| types.push(enemy.kind_type.clone()));

        if !invalids.is_empty() {
            panic!(
                "{} Invalid enemies generated from sample size of {}.\nTypes generated: {:?}",
                invalids.len(),
                num_enemies,
                types
            );
        }
    }
}
