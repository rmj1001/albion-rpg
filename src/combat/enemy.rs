use crate::{
    data::{player::Player, xp::XP},
    utils::math::{self, random_num},
};

#[derive(Clone, Debug)]
pub enum Enemy {
    Animal(Animal),
    Human,
    Monster(Monster),
    Undead(Undead),
    Invalid,
}

#[derive(Clone, Debug)]
pub enum Animal {
    Bear,
    DireWolf,
    GiantSpider,
    WhiteApe,
    Owlbear,
    Wyrm,
    Invalid,
}

#[derive(Clone, Debug)]
pub enum Monster {
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
pub enum Undead {
    Banshee,
    Ghost,
    Skeleton,
    Vampire,
    Zombie,
    Invalid,
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
        let kind_string: &str = EnemyData::kind_string(kind_type.clone());

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

    pub fn kind_string(kind: Enemy) -> &'static str {
        match kind {
            Enemy::Human => "Human",
            Enemy::Invalid => "Invalid Enemy",
            Enemy::Animal(animal) => match animal {
                Animal::Bear => "Bear",
                Animal::Wyrm => "Wyrm",
                Animal::Invalid => "Invalid Animal",
                Animal::Owlbear => "Owlbear",
                Animal::DireWolf => "Dire Wolf",
                Animal::WhiteApe => "White Ape",
                Animal::GiantSpider => "Giant Spider",
            },
            Enemy::Undead(undead) => match undead {
                Undead::Banshee => "Banshee",
                Undead::Ghost => "Ghost",
                Undead::Zombie => "Zombie",
                Undead::Invalid => "Invalid Undead",
                Undead::Vampire => "Vampire",
                Undead::Skeleton => "Skeleton",
            },
            Enemy::Monster(monster) => match monster {
                Monster::Centaur => "Centaur",
                Monster::Orc => "Orc",
                Monster::Giant => "Giant",
                Monster::Troll => "Troll",
                Monster::Dragon => "Dragon",
                Monster::Goblin => "Goblin",
                Monster::DarkElf => "Dark Elf",
                Monster::Invalid => "Invalid Monster",
                Monster::Werewolf => "Werewolf",
            },
        }
    }
}

fn pick_enemy() -> Enemy {
    let number = math::random_num(0, 3);

    match number {
        0 => Enemy::Animal(pick_animal()),
        1 => Enemy::Human,
        2 => Enemy::Monster(pick_monster()),
        3 => Enemy::Undead(pick_undead()),
        _ => Enemy::Invalid,
    }
}

fn pick_animal() -> Animal {
    let number: usize = math::random_num(0, 5);

    match number {
        0 => Animal::Bear,
        1 => Animal::DireWolf,
        2 => Animal::GiantSpider,
        3 => Animal::Owlbear,
        4 => Animal::WhiteApe,
        5 => Animal::Wyrm,
        _ => Animal::Invalid,
    }
}

fn pick_monster() -> Monster {
    let number: usize = math::random_num(0, 7);

    match number {
        0 => Monster::Centaur,
        1 => Monster::DarkElf,
        2 => Monster::Dragon,
        3 => Monster::Giant,
        4 => Monster::Goblin,
        5 => Monster::Orc,
        6 => Monster::Troll,
        7 => Monster::Werewolf,
        _ => Monster::Invalid,
    }
}

fn pick_undead() -> Undead {
    let number: usize = math::random_num(0, 4);

    match number {
        0 => Undead::Banshee,
        1 => Undead::Ghost,
        2 => Undead::Skeleton,
        3 => Undead::Vampire,
        4 => Undead::Zombie,
        _ => Undead::Invalid,
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
        use crate::combat::enemy::{Animal, Enemy, EnemyData, Monster, Undead};
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

        let invalid_enemy_filter = |enemy: &&EnemyData| {
            matches!(
                enemy.kind_type,
                Enemy::Invalid
                    | Enemy::Animal(Animal::Invalid)
                    | Enemy::Monster(Monster::Invalid)
                    | Enemy::Undead(Undead::Invalid)
            )
        };

        let invalids: Vec<EnemyData> = enemies.iter().filter(invalid_enemy_filter).cloned().collect();
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
