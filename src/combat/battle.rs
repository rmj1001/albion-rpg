use crate::{
    combat::enemy::{add_rewards_to_user, generate_rewards, Enemy},
    player::profile::UserProfile,
    utils::{
        input,
        math::random_num,
        messages::out_of_bounds,
        tui::{self, page_header, press_enter_to_continue, sleep, HeaderSubtext},
    },
};

use super::inventory::battle_inventory;

pub fn battle(player: &mut UserProfile, looped: bool) {
    // Prelude
    page_header("Battle", HeaderSubtext::None);

    let mut current_enemy: Enemy = Enemy::new(player.xp.combat, player.health.hp);

    println!("You are now fighting a {}.", current_enemy.type_string());
    tui::press_enter_to_continue();
    battle_menu(player, &mut current_enemy, looped)
}

pub fn battle_menu(player: &mut UserProfile, enemy: &mut Enemy, looped: bool) {
    page_header(format!("Battle - {}", enemy.type_string()), HeaderSubtext::Keyboard);

    if player.health.hp < 100 && player.health.hunger == 0 {
        let new_health: usize = random_num(0, 1);

        if new_health != 0 {
            println!("You gained {} health!", new_health);
            press_enter_to_continue();
            page_header(format!("Battle - {}", enemy.type_string()), HeaderSubtext::Keyboard);
        }
    }

    let action = input::select_from_str_array(
        &[
            &format!("1. Attack the {}", enemy.type_string()),
            "Inventory",
            "Retreat",
        ],
        None,
    );

    match action {
        0 => attack(player, enemy, looped),
        1 => {
            battle_inventory(player);
            battle_menu(player, enemy, looped);
        }
        2 => retreat(player),
        _ => out_of_bounds(),
    }
}

pub fn retreat(player: &mut UserProfile) {
    page_header("Battle - Retreat", HeaderSubtext::None);

    println!("You have retreated from the battle.");
    press_enter_to_continue();

    crate::menus::game_menu::main(player);
}

pub fn attack(player: &mut UserProfile, enemy: &mut Enemy, looped: bool) {
    page_header("Battle", HeaderSubtext::None);

    player_attack(player, enemy, looped);
    enemy_attack(player, enemy);
    battle_menu(player, enemy, looped);
}

fn player_attack(player: &mut UserProfile, enemy: &mut Enemy, looped: bool) {
    let enemy_type = enemy.type_string();

    println!("You attack the {}...", enemy_type);
    sleep(1);

    let hit = success_or_fail();

    if hit {
        println!("You hit the {}!", enemy_type);

        // TODO: Calculate enemy damage based on armor

        if enemy.hp == 0 {
            victory(player, enemy, looped);
        }
    } else {
        println!("You missed the {}.", enemy_type);
    }

    sleep(1);
}

fn enemy_attack(player: &mut UserProfile, enemy: &mut Enemy) {
    let enemy_type = enemy.type_string();

    println!("The {} attacks you...", enemy_type);
    sleep(1);

    let hit = success_or_fail();

    if hit {
        println!("The {} hit you!!", enemy_type);

        // TODO: Calculate player damage based on enemy damage and equipped armor strength

        if player.health.hp == 0 {
            defeat(player, enemy);
        }
    } else {
        println!("The {} missed you.", enemy_type);
    }

    sleep(1);
}

fn success_or_fail() -> bool {
    let num = random_num(0, 1);

    num == 0
}

pub fn victory(player: &mut UserProfile, enemy: &mut Enemy, looped: bool) {
    page_header("Battle - Victory", HeaderSubtext::None);

    println!("You successfully defeated the {}!", enemy.type_string());
    player.reset_health();

    let rewards: Vec<crate::combat::enemy::Rewards> = generate_rewards(player.xp.profile_level());

    println!("Your rewards:\n{:?}", &rewards);
    add_rewards_to_user(player, rewards);

    press_enter_to_continue();

    if !looped {
        crate::menus::game_menu::main(player);
    }
}

pub fn defeat(player: &mut UserProfile, enemy: &mut Enemy) {
    page_header("Battle - Defeat", HeaderSubtext::None);

    println!("You have been defeated in battle.");
    tui::sleep(1);

    println!("The {} stole all your gold and inventory.", enemy.type_string());
    player.reset_inventory();
    tui::sleep(1);

    println!("You have been rushed to the local physician.");
    tui::sleep(1);

    if player.settings.hardmode {
        hardmode(player);
    } else {
        revived(player);
    }
}

pub fn revived(player: &mut UserProfile) {
    println!("You were successfully revived with 100 hp.");
    player.reset_health();

    press_enter_to_continue();
    crate::menus::game_menu::main(player);
}

pub fn hardmode(player: &mut UserProfile) {
    let user_survives = random_num(0, 1);

    match user_survives {
        0 => {
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
