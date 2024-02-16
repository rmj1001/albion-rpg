use crate::{
    data::{inventory::equipment::Equipment, player::Player},
    utils::{
        input::select_from_str_array,
        math::random_num,
        messages::out_of_bounds,
        tui::{page_header, press_enter_to_continue, HeaderSubtext},
    },
};

pub fn battle_inventory(player: &mut Player) {
    page_header("Battle Inventory", HeaderSubtext::Keyboard);

    let choice: usize = select_from_str_array(&["1. Equipment", "2. Healing", "NAV: Go Back"], None);

    match choice {
        0 => {
            Equipment::menu(player);
            battle_inventory(player);
        }
        1 => healing_inventory(player),
        2 => {} // just returns to battle menu since the battle menu function is recursive called after this menu
        _ => out_of_bounds(),
    }
}

pub fn healing_inventory(player: &mut Player) {
    page_header("Healing Inventory", HeaderSubtext::Keyboard);

    let choice: usize = select_from_str_array(&["1. Use Potion", "2. Eat Food", "NAV: Go Back"], None);

    match choice {
        0 => {
            use_potion(player);
            healing_inventory(player);
        }
        1 => {
            eat_food(player);
            healing_inventory(player);
        }
        2 => battle_inventory(player),
        _ => out_of_bounds(),
    }
}

pub fn use_potion(player: &mut Player) {
    if player.items.potions == 0 {
        println!("You do not have enough potions.");
        press_enter_to_continue();
        return;
    }

    player.items.potions -= 1;

    let health = random_num(1, 5);
    player.health.hp += health;

    println!("Your health increased {} hp, and is now {}.", health, player.health.hp);
    press_enter_to_continue();
}

pub fn eat_food(player: &mut Player) {
    if player.items.food == 0 {
        println!("You do not have enough food.");
        press_enter_to_continue();
        return;
    }

    player.items.food -= 1;

    let hunger = random_num(1, 5);
    player.health.hunger -= hunger;

    println!(
        "Your hunger decreased {} points, and is now {}.",
        hunger, player.health.hunger
    );
    press_enter_to_continue();
}
