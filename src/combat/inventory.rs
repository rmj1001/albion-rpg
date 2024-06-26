use crate::{
    data::{inventory::equipment::Equipment, player::Player},
    prelude::{page_header, pause, random_num, select, unreachable, Instructions},
};

/// Choose between equipment and healing
pub fn battle_menu(player: &mut Player) {
    page_header("Battle Inventory", &Instructions::Keyboard);

    let choice: usize = select(&["1. Equipment", "2. Healing", "NAV: Go Back"], None);

    match choice {
        0 => {
            Equipment::menu(player);
            battle_menu(player);
        }
        1 => healing_menu(player),
        2 => {} // just returns to battle menu since the battle menu function is recursive called after this menu
        _ => unreachable(),
    }
}

pub fn healing_menu(player: &mut Player) {
    page_header("Healing Inventory", &Instructions::Keyboard);

    println!("Potions: {}", player.items.potions);
    println!("Food: {}", player.items.food);
    println!();

    let choice: usize = select(&["1. Use Potion", "2. Eat Food", "NAV: Go Back"], None);

    match choice {
        0 => {
            use_potion(player);
            healing_menu(player);
        }
        1 => {
            eat_food(player);
            healing_menu(player);
        }
        2 => battle_menu(player),
        _ => unreachable(),
    }
}

pub fn use_potion(player: &mut Player) {
    if player.items.potions == 0 {
        println!("You do not have enough potions.");
        pause();
        return;
    }

    player.items.potions -= 1;

    let health = random_num(1, 5);
    player.health.hp += health;

    println!("Your health increased {} hp, and is now {}.", health, player.health.hp);
    pause();
}

pub fn eat_food(player: &mut Player) {
    if player.items.food == 0 {
        println!("You do not have enough food.");
        pause();
        return;
    }

    player.items.food -= 1;

    let hunger = random_num(1, 5);
    player.health.hunger -= hunger;

    println!(
        "Your hunger decreased {} points, and is now {}.",
        hunger, player.health.hunger
    );
    pause();
}
