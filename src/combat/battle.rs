use crate::{
    combat::enemy::{add_rewards_to_user, generate_rewards, Enemy, Rewards},
    player::{equipment::Equipment, profile::Player},
    utils::{
        input,
        math::random_num,
        messages::out_of_bounds,
        tui::{self, page_header, press_enter_to_continue, sleep, HeaderSubtext},
    },
};

use super::inventory::battle_inventory;

pub fn battle(header: &'static str, prompt: &'static str, player: &mut Player, looped: bool) {
    // Prelude
    page_header(header, HeaderSubtext::None);

    if player.equipment.armor.is_none() || player.equipment.weapon.is_none() {
        let confirm = input::confirm("Are you sure you want to fight without equipment? You'll die.");

        if !confirm {
            println!("Returning home.");
            press_enter_to_continue();

            crate::menus::game_menu::main(player);
        }
    }

    println!("{}", prompt);
    sleep(3);

    let mut current_enemy: Enemy = Enemy::new(player.xp.combat, player.health.hp);

    println!("You are now fighting a {}.", current_enemy.type_string());
    tui::press_enter_to_continue();
    battle_menu(header, player, &mut current_enemy, looped)
}

pub fn battle_menu(header: &'static str, player: &mut Player, enemy: &mut Enemy, looped: bool) {
    page_header(format!("{} - {}", header, enemy.type_string()), HeaderSubtext::Keyboard);

    if player.health.hp < 100 && player.health.hunger == 0 {
        let new_health: usize = random_num(0, 1);

        if new_health != 0 {
            println!("You gained {} health!", new_health);
            press_enter_to_continue();
            page_header(format!("Battle - {}", enemy.type_string()), HeaderSubtext::Keyboard);
        }
    }

    println!("Enemy: {}", enemy.type_string());
    println!("Enemy HP: {}", enemy.hp);
    println!();
    println!("Player HP: {}", player.health.hp);
    println!("Player Hunger: {}", player.health.hunger);
    println!();

    let action = input::select_from_str_array(
        &[
            &format!("1. Attack the {}", enemy.type_string()),
            "2. Inventory",
            "3. Retreat",
        ],
        None,
    );

    match action {
        0 => attack(header, player, enemy, looped),
        1 => {
            battle_inventory(player);
            battle_menu(header, player, enemy, looped);
        }
        2 => retreat(player),
        _ => out_of_bounds(),
    }
}

pub fn retreat(player: &mut Player) {
    page_header("Battle - Retreat", HeaderSubtext::None);

    println!("You have retreated from the battle.");
    press_enter_to_continue();

    crate::menus::game_menu::main(player);
}

pub fn attack(header: &'static str, player: &mut Player, enemy: &mut Enemy, looped: bool) {
    page_header(header, HeaderSubtext::None);

    player_attack(header, player, enemy, looped);
    enemy_attack(header, player, enemy);
    battle_menu(header, player, enemy, looped);
}

fn player_attack(header: &'static str, player: &mut Player, enemy: &mut Enemy, looped: bool) {
    let enemy_type = enemy.type_string();

    println!("You attack the {}...", enemy_type);
    sleep(1);

    let hit = success_or_fail();

    if hit && player.equipment.weapon.is_some() {
        let mut weapon = player.equipment.weapon.clone().unwrap();
        let damage = weapon.damage;

        println!("You hit the {} for {} damage!", enemy_type, damage);

        weapon.decrease_durability();

        if !weapon.owns {
            player.equipment.weapon = None;
        } else {
            player.equipment.weapon = Some(weapon.clone());
        }

        Equipment::overwrite_inventory_weapon(weapon, player);

        if enemy.hp < damage {
            victory(header, player, enemy, looped);
        } else {
            enemy.hp -= damage;
        }
    } else {
        println!("You missed the {}.", enemy_type);
    }

    sleep(1);
}

fn enemy_attack(header: &'static str, player: &mut Player, enemy: &mut Enemy) {
    let enemy_type = enemy.type_string();
    let mut damage: usize = enemy.damage;

    if player.equipment.armor.is_some() {
        let armor = player.equipment.armor.clone().unwrap();

        if damage > armor.defense {
            damage -= armor.defense;
        } else {
            damage = 0
        }

        let mut new_armor = player.equipment.armor.clone().unwrap();
        new_armor.decrease_durability();

        if !new_armor.owns {
            player.equipment.armor = None;
        } else {
            player.equipment.armor = Some(new_armor.clone());
        }

        Equipment::overwrite_inventory_armor(new_armor, player);
    }

    println!("The {} attacks you...", enemy_type);
    sleep(1);

    let hit = success_or_fail();

    if hit && damage > 0 {
        println!("The {} hit you for {} damage!!", enemy_type, damage);

        if player.health.hp < damage {
            defeat(header, player, enemy);
        } else {
            player.health.hp -= damage;
        }
    } else if damage == 0 {
        println!("The {} hit but the damage was negated by your armor!", enemy_type);
    } else {
        println!("The {} missed you.", enemy_type);
    }

    sleep(1);
}

fn success_or_fail() -> bool {
    let num = random_num(0, 1);

    num == 0
}

pub fn victory(header: &'static str, player: &mut Player, enemy: &mut Enemy, looped: bool) {
    page_header(format!("{} - Victory", header), HeaderSubtext::None);

    println!("You successfully defeated the {}!", enemy.type_string());
    player.reset_health();
    println!();

    let rewards: Vec<Rewards> = generate_rewards(player.xp.profile_level());

    println!("Items Looted:");

    for reward in &rewards {
        println!("- {:?}", reward)
    }

    add_rewards_to_user(player, rewards);
    println!();

    println!("Gained Combat XP: {}", enemy.xp);
    player.xp.combat += enemy.xp;
    println!("Total Combat XP: {}", player.xp.combat);
    println!();

    println!("Gained Gold: {}", enemy.gold);
    player.bank.wallet += enemy.gold;
    println!("Total Gold: {}", player.bank.wallet);
    println!();

    press_enter_to_continue();
    player.save();

    if !looped {
        crate::menus::game_menu::main(player);
    }
}

pub fn defeat(header: &'static str, player: &mut Player, enemy: &mut Enemy) {
    page_header(format!("{} - Defeat", header), HeaderSubtext::None);

    println!("You have been defeated in battle.");
    tui::sleep(1);

    println!("You have been rushed to the local physician.");
    tui::sleep(1);

    if player.settings.hardmode {
        hardmode(player, enemy);
    } else {
        revived(player);
    }
}

pub fn revived(player: &mut Player) {
    println!("You were successfully revived with 100 hp.");
    player.reset_health();

    player.save();
    press_enter_to_continue();
    crate::menus::game_menu::main(player);
}

pub fn hardmode(player: &mut Player, enemy: &mut Enemy) {
    let user_survives = random_num(0, 1);

    match user_survives {
        0 => {
            println!("The {} stole all your gold and inventory.", enemy.type_string());
            player.reset_inventory();
            player.save();
            tui::sleep(1);

            revived(player);
        }
        1 => {
            println!("You didn't survive. This profile will be deleted.");
            press_enter_to_continue();

            player.delete();

            crate::menus::accounts::main();
        }
        _ => out_of_bounds(),
    }
}
