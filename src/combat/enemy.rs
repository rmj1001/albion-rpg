use crate::{
    data::{player::Player, xp::XP},
    utils::math::{self, random_num},
};

#[derive(Clone, Debug)]
pub enum EnemyType {
    Animal(AnimalType),
    Human,
    Monster(MonsterType),
    Undead(UndeadType),
    Invalid,
}

#[derive(Clone, Debug)]
pub enum AnimalType {
    Bear,
    DireWolf,
    GiantSpider,
    WhiteApe,
    Owlbear,
    Wyrm,
    Invalid,
}

#[derive(Clone, Debug)]
pub enum MonsterType {
    Centaur,
    DarkElf,
    Dragon,
    Giant,
    Goblin,
    Orc,
    Troll,
    Werewolf,
    Invalid,
}

#[derive(Clone, Debug)]
pub enum UndeadType {
    Banshee,
    Ghost,
    Skeleton,
    Vampire,
    Zombie,
    Invalid,
}

#[derive(Clone)]
pub struct Enemy {
    pub kind_type: EnemyType,
    pub kind: &'static str,
    pub hp: usize,
    pub damage: usize,
    pub xp: usize,
    pub gold: usize,
    pub rewards: Vec<Rewards>,
}

impl Enemy {
    pub fn new(user_combat_xp: usize, user_hp: usize) -> Self {
        let user_level: usize = XP::get_level(user_combat_xp);
        let kind_type: EnemyType = pick_enemy();
        let kind_string: &str = Enemy::kind_string(kind_type.clone());

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

    pub fn kind_string(kind: EnemyType) -> &'static str {
        match kind {
            EnemyType::Human => "Human",
            EnemyType::Invalid => "Invalid Enemy",
            EnemyType::Animal(animal) => match animal {
                AnimalType::Bear => "Bear",
                AnimalType::Wyrm => "Wyrm",
                AnimalType::Invalid => "Invalid Animal",
                AnimalType::Owlbear => "Owlbear",
                AnimalType::DireWolf => "Dire Wolf",
                AnimalType::WhiteApe => "White Ape",
                AnimalType::GiantSpider => "Giant Spider",
            },
            EnemyType::Undead(undead) => match undead {
                UndeadType::Banshee => "Banshee",
                UndeadType::Ghost => "Ghost",
                UndeadType::Zombie => "Zombie",
                UndeadType::Invalid => "Invalid Undead",
                UndeadType::Vampire => "Vampire",
                UndeadType::Skeleton => "Skeleton",
            },
            EnemyType::Monster(monster) => match monster {
                MonsterType::Centaur => "Centaur",
                MonsterType::Orc => "Orc",
                MonsterType::Giant => "Giant",
                MonsterType::Troll => "Troll",
                MonsterType::Dragon => "Dragon",
                MonsterType::Goblin => "Goblin",
                MonsterType::DarkElf => "Dark Elf",
                MonsterType::Invalid => "Invalid Monster",
                MonsterType::Werewolf => "Werewolf",
            },
        }
    }
}

fn pick_enemy() -> EnemyType {
    let number = math::random_num(0, 3);

    match number {
        0 => EnemyType::Animal(pick_animal()),
        1 => EnemyType::Human,
        2 => EnemyType::Monster(pick_monster()),
        3 => EnemyType::Undead(pick_undead()),
        _ => EnemyType::Invalid,
    }
}

fn pick_animal() -> AnimalType {
    let number: usize = math::random_num(0, 5);

    match number {
        0 => AnimalType::Bear,
        1 => AnimalType::DireWolf,
        2 => AnimalType::GiantSpider,
        3 => AnimalType::Owlbear,
        4 => AnimalType::WhiteApe,
        5 => AnimalType::Wyrm,
        _ => AnimalType::Invalid,
    }
}

fn pick_monster() -> MonsterType {
    let number: usize = math::random_num(0, 7);

    match number {
        0 => MonsterType::Centaur,
        1 => MonsterType::DarkElf,
        2 => MonsterType::Dragon,
        3 => MonsterType::Giant,
        4 => MonsterType::Goblin,
        5 => MonsterType::Orc,
        6 => MonsterType::Troll,
        7 => MonsterType::Werewolf,
        _ => MonsterType::Invalid,
    }
}

fn pick_undead() -> UndeadType {
    let number: usize = math::random_num(0, 4);

    match number {
        0 => UndeadType::Banshee,
        1 => UndeadType::Ghost,
        2 => UndeadType::Skeleton,
        3 => UndeadType::Vampire,
        4 => UndeadType::Zombie,
        _ => UndeadType::Invalid,
    }
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
        rewards.push(Rewards::MagicScrolls(random_num(0, 3)));
    }

    if player_level > 25 {
        rewards.push(Rewards::DragonHides(random_num(0, 3)));
    }

    if player_level > 50 {
        rewards.push(Rewards::Rubies(random_num(0, 3)));
    }

    if player_level > 100 {
        rewards.push(Rewards::RunicTablets(random_num(0, 3)));
    }

    rewards
}

pub fn add_rewards_to_user(player: &mut Player, rewards: Vec<Rewards>) {
    for reward in rewards {
        match reward {
            Rewards::Potions(quantity) => player.items.potions += quantity,
            Rewards::Bones(quantity) => player.items.bones += quantity,
            Rewards::Rubies(quantity) => player.items.rubies += quantity,
            Rewards::DragonHides(quantity) => player.items.dragon_hides += quantity,
            Rewards::MagicScrolls(quantity) => player.items.magic_scrolls += quantity,
            Rewards::RunicTablets(quantity) => player.items.runic_tablets += quantity,
            Rewards::Invalid => {}
        }
    }
}

pub mod tests {
    #[test]
    pub fn invalid_enemies() {
        use crate::combat::enemy::{AnimalType, Enemy, EnemyType, MonsterType, UndeadType};
        use crate::data::player::Player;
        use crate::utils::crypt;

        let test_player = Player::new("test", &crypt::generate_hash("test".to_string()), false);

        let num_enemies: usize = 500;
        let mut enemies: Vec<Enemy> = vec![];

        let mut index: usize = 0;

        while index < num_enemies {
            enemies.push(Enemy::new(test_player.xp.combat, test_player.health.hp));
            index += 1
        }

        let invalid_enemy_filter = |enemy: &&Enemy| {
            matches!(
                enemy.kind_type,
                EnemyType::Invalid
                    | EnemyType::Animal(AnimalType::Invalid)
                    | EnemyType::Monster(MonsterType::Invalid)
                    | EnemyType::Undead(UndeadType::Invalid)
            )
        };

        let invalids: Vec<Enemy> = enemies.iter().filter(invalid_enemy_filter).cloned().collect();
        let mut types: Vec<EnemyType> = vec![];

        invalids
            .iter()
            .for_each(|enemy: &Enemy| types.push(enemy.kind_type.clone()));

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
