use crate::{
    combat::enemy::{EnemyData, Rewards},
    data::{inventory::equipment::Equipment, player::Player, xp::XP},
    panic_menu,
    prelude::*,
};

pub struct BattleSettings<'a> {
    pub header: &'static str,
    pub prompt: &'static str,
    pub player: &'a mut Player,
    pub enemy: EnemyData,
    pub loops: usize,
    pub floor: usize,
    pub is_first_battle: bool,
    pub is_looped: bool,
    pub pause_seconds: u64,
    pub end_function: Option<fn(&mut Player)>,
}

use super::inventory::battle_inventory;

/// Entry point for starting a battle.
pub fn new_battle(battle: &mut BattleSettings) {
    // Prelude
    page_header(battle.header, Instructions::None);
    Equipment::check_equipment_ownership(battle.player);

    if battle.player.equipment.armor.is_none() || battle.player.equipment.weapon.is_none() {
        let confirm = confirm("Are you sure you want to fight without equipment? You'll die.");

        if !confirm {
            println!("Returning home.");
            pause();

            crate::menus::game_menu::main(battle.player);
        }
    }

    if battle.loops > 0 {
        battle.floor += 1;
        battle.loops -= 1;
    }

    println!("{}", battle.prompt);
    sleep(battle.pause_seconds);

    if !battle.is_first_battle {
        battle.enemy = EnemyData::new(battle.player.xp.combat, battle.player.health.hp);
    } else {
        battle.is_first_battle = false; // generate new enemy for subsequent battles
    }

    println!();
    println!("You are now fighting a {}.", battle.enemy.name);
    sleep(battle.pause_seconds);
    battle_menu(battle);
}

pub fn battle_menu(battle: &mut BattleSettings) {
    page_header(
        format!("{} - {}", battle.header, battle.enemy.name),
        Instructions::Keyboard,
    );

    if battle.is_looped {
        println!("Floor: {}", battle.floor);
        println!("Floors Left: {}", battle.loops);
        println!();
    }

    println!("Enemy: {}", battle.enemy.name);
    println!("Enemy HP: {}", battle.enemy.hp);
    println!();

    println!("Player HP: {}", battle.player.health.hp);
    println!("Player Hunger: {}", battle.player.health.hunger);
    println!();

    let attack_string = &format!("1. Attack the {}", battle.enemy.name);

    let action = select(&[attack_string.as_str(), "2. Inventory", "3. Retreat"], None);

    match action {
        0 => attack(battle),
        1 => {
            battle_inventory(battle.player);
            battle_menu(battle);
        }
        2 => retreat(battle.player),
        _ => out_of_bounds(),
    }
}

pub fn retreat(player: &mut Player) {
    page_header("Battle - Retreat", Instructions::None);

    println!("You have retreated from the battle.");
    pause();

    crate::menus::game_menu::main(player);
}

pub fn attack(battle: &mut BattleSettings) {
    page_header(battle.header, Instructions::None);

    player_attack(battle);

    println!();

    enemy_attack(battle);

    println!();

    battle.player.health.heal();

    println!();

    pause();

    battle_menu(battle);
}

fn player_attack(battle: &mut BattleSettings) {
    let enemy_type = &battle.enemy.name;

    println!("You attack the {}...", enemy_type);
    sleep(battle.pause_seconds);

    let hit = success_or_fail();

    if !hit || battle.player.equipment.weapon.is_none() {
        println!("You missed the {}.", enemy_type);
        sleep(battle.pause_seconds);
        return;
    }

    if let Some(equipped_weapon) = &battle.player.equipment.weapon {
        let weapon = battle.player.weapons.get(equipped_weapon);
        let damage = weapon.damage;

        println!("You hit the {} for {} damage!", enemy_type, damage);

        weapon.decrease_durability();

        if !weapon.owns {
            Equipment::unequip_weapon(battle.player, false);
        }

        if battle.enemy.hp < damage {
            victory(battle);
        } else {
            battle.enemy.hp -= damage;
        }
    }

    sleep(battle.pause_seconds);
}

fn enemy_attack(battle: &mut BattleSettings) {
    let enemy_type = &battle.enemy.name;
    let mut damage: usize = battle.enemy.damage;

    if let Some(equipped_armor) = &battle.player.equipment.armor {
        let armor = battle.player.armor.get(equipped_armor);

        if damage > armor.defense {
            damage -= armor.defense;
        } else {
            damage = 0
        }

        armor.decrease_durability();

        if !armor.owns {
            Equipment::unequip_armor(battle.player, false);
        }
    }

    println!("The {} attacks you...", enemy_type);
    sleep(battle.pause_seconds);

    let hit = success_or_fail();

    if hit && damage > 0 {
        println!("The {} hit you for {} damage!!", enemy_type, damage);

        if battle.player.health.hp < damage {
            defeat(battle);
        } else {
            battle.player.health.hp -= damage;
        }
    } else if damage == 0 {
        println!("The {} hit but the damage was negated by your armor!", enemy_type);
    } else {
        println!("The {} missed you.", enemy_type);
    }

    sleep(battle.pause_seconds);
}

fn success_or_fail() -> bool {
    let num = random_num(0, 1);

    num == 0
}

pub fn victory(battle: &mut BattleSettings) {
    page_header(format!("{} - Victory", battle.header), Instructions::None);

    println!("You successfully defeated the {}!", battle.enemy.name);
    battle.player.health.restore();
    battle.player.achievements.monsters_killed += 1;
    println!();

    let rewards = Rewards::new(XP::get_level(battle.player.xp.total()));

    println!("Items Looted:");

    rewards.iter().for_each(|reward| {
        println!("- {}", reward);
    });

    Rewards::reward_to_player(battle.player, rewards);
    println!();

    pause();
    battle.player.save();

    if !battle.is_looped {
        crate::menus::game_menu::main(battle.player);
    }

    if battle.loops > 0 {
        new_battle(battle);
    }

    if let Some(end_func) = battle.end_function {
        end_func(battle.player);
    }
}

pub fn defeat(battle: &mut BattleSettings) {
    page_header(format!("{} - Defeat", battle.header), Instructions::None);

    println!("You have been defeated in battle.\n");
    sleep(battle.pause_seconds);

    println!("You have been rushed to the local physician.\n");
    sleep(battle.pause_seconds);

    if battle.player.settings.hardmode {
        hardmode(battle);
    } else {
        revived(battle);
    }
}

pub fn revived(battle: &mut BattleSettings) {
    println!("You were successfully revived with 100 hp.\n");
    battle.player.health.reset();

    battle.player.save();
    pause();
    crate::menus::game_menu::main(battle.player);
}

/// Result of battle if player defeated and hardmode is enabled.
pub fn hardmode(battle: &mut BattleSettings) {
    let user_survives = random_num(0, 1);

    match user_survives {
        0 => {
            println!(
                "The {} stole all your gold and inventory, and you lost all your progress.\n",
                battle.enemy.name
            );
            battle.player.die();
            sleep(battle.pause_seconds);

            revived(battle);
        }
        1 => {
            println!("You didn't survive. This profile will be deleted.\n");
            pause();

            match battle.player.delete() {
                Ok(_) => crate::menus::accounts::main(),
                Err(error) => panic_menu!(error),
            }
        }
        _ => out_of_bounds(),
    }
}
