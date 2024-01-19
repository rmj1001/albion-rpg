use rand::Rng;

use crate::user::xp::XP;

pub enum EnemyType {
    Animal(AnimalType),
    Human,
    Monster(MonsterType),
    Undead(UndeadType),
}

pub enum AnimalType {
    Bear,
    DireWolf,
    GiantSpider,
}

pub enum MonsterType {
    Goblin,
    Orc,
    DarkElf,
    Dragon,
}

pub enum UndeadType {
    Ghost,
    Skeleton,
    Vampire,
    Zombie,
}

pub enum Rewards {
    Potions(usize),
    Rubies(usize),
    MagicScrolls(usize),
    Bones(usize),
    DragonHides(usize),
    RunicTablets(usize),
}

pub struct Enemy {
    pub kind: EnemyType,
    pub hp: usize,
    pub damage: usize,
    pub xp: usize,
    pub gold: usize,
    pub rewards: Vec<Rewards>,
}

impl Enemy {
    pub fn new(combat_xp: usize) -> Self {
        let user_level = XP::level(combat_xp);

        Self {
            kind: pick_enemy(),
            hp: calculate_hp(user_level),
            damage: calculate_damage(user_level),
            xp: calculate_xp(user_level),
            gold: calculate_gold(user_level),
            rewards: generate_rewards(user_level),
        }
    }
}

// Enemy type

fn pick_enemy() -> EnemyType {
    let kind_number = rand::thread_rng().gen_range(1..4);

    match kind_number {
        1 => EnemyType::Animal(pick_animal()),
        2 => EnemyType::Human,
        3 => EnemyType::Monster(pick_monster()),
        4 => EnemyType::Undead(pick_undead()),
    }
}

fn pick_animal() -> AnimalType {}

fn pick_monster() -> MonsterType {}

fn pick_undead() -> UndeadType {}

// Strength

fn calculate_hp(player_level: usize) -> usize {}

fn calculate_damage(player_level: usize) -> usize {}

// Rewards

fn calculate_xp(player_level: usize) -> usize {}

fn calculate_gold(player_level: usize) -> usize {}

fn generate_rewards(player_level: usize) -> Vec<Rewards> {}
