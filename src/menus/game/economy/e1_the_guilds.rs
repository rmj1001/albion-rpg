#![allow(unused_assignments, unused_variables, unused_mut)]
use albion_termrpg::lib::input::selector;

use albion_termrpg::lib::{
    tui::{self, page_header},
    user::inventory::{GuildItemNames, Item},
    user::profile::UserProfile,
    user::xp::{XPType, XP},
};
use rand::Rng;

pub fn main_menu(user: &mut UserProfile) {
    page_header(
        "The Guilds",
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    let job_name: String;
    let job_xp: XPType;
    let use_gold: bool;
    let mut increase_item: Option<&mut Item>;
    let mut decrease_item: Option<&mut Item>;

    let guild_choice = selector(
        &[
            "1. Fishing",
            "2. Cooking",
            "3. Woodcutting",
            "4. Mining",
            "5. Smithing",
            "6. Thieving",
            "NAV: Go Back",
        ],
        0,
        Some(""),
    );

    match guild_choice {
        0 => job(
            user,
            "Fishing".to_string(),
            XPType::Fishing,
            false,
            &Some(GuildItemNames::Fish),
            &None,
        ),
        1 => job(
            user,
            "Cooking".to_string(),
            XPType::Cooking,
            false,
            &Some(GuildItemNames::CookedFish),
            &Some(GuildItemNames::Fish),
        ),
        2 => job(
            user,
            "Woodcutting".to_string(),
            XPType::Woodcutting,
            false,
            &Some(GuildItemNames::Wood),
            &None,
        ),
        3 => job(
            user,
            "Mining".to_string(),
            XPType::Mining,
            false,
            &Some(GuildItemNames::Ore),
            &None,
        ),
        4 => job(
            user,
            "Smithing".to_string(),
            XPType::Smithing,
            false,
            &Some(GuildItemNames::Ingots),
            &Some(GuildItemNames::Ore),
        ),
        5 => job(
            user,
            "Thieving".to_string(),
            XPType::Thieving,
            true,
            &None,
            &None,
        ),
        6 => crate::menus::game::main::menu(user),
        _ => panic!("Dialogue selector picked invalid index."),
    }
}

fn job(
    user: &mut UserProfile,
    job_name: String,
    xp_type: XPType,
    use_gold: bool,
    increase_item: &Option<GuildItemNames>,
    decrease_item: &Option<GuildItemNames>,
) {
    page_header(
        &format!("Job: {}", job_name),
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    println!("XP: {}", user.xp.get(xp_type));
    println!("Level: {}", XP::level(user.xp.get(xp_type)));

    if use_gold {
        println!();
        println!("Gold: {}", user.gold);
    }

    println!();

    match increase_item {
        Some(item) => match item {
            GuildItemNames::CookedFish => {
                println!("Cooked Fish: {}", user.inventory.food.quantity)
            }
            GuildItemNames::Fish => println!("Fish: {}", user.inventory.fish.quantity),
            GuildItemNames::Wood => println!("Wood: {}", user.inventory.wood.quantity),
            GuildItemNames::Ingots => println!("Ingots: {}", user.inventory.ingots.quantity),
            GuildItemNames::Ore => println!("Ores: {}", user.inventory.ore.quantity),
        },
        None => {}
    }

    match decrease_item {
        Some(item) => match item {
            GuildItemNames::CookedFish => {
                println!("Cooked Fish: {}", user.inventory.food.quantity)
            }
            GuildItemNames::Fish => println!("Fish: {}", user.inventory.fish.quantity),
            GuildItemNames::Wood => println!("Wood: {}", user.inventory.wood.quantity),
            GuildItemNames::Ingots => println!("Ingots: {}", user.inventory.ingots.quantity),
            GuildItemNames::Ore => println!("Ores: {}", user.inventory.ore.quantity),
        },
        None => {}
    }

    println!();

    let work_choice = selector(&["Work", "Go Back"], 0, Some(""));

    match work_choice {
        0 => {
            user.xp.increment(xp_type);

            if use_gold {
                user.gold += rand::thread_rng().gen_range(0..2);

                job(
                    user,
                    job_name.clone(),
                    xp_type,
                    use_gold,
                    increase_item,
                    decrease_item,
                );
            }

            match increase_item {
                Some(item) => match item {
                    GuildItemNames::CookedFish => user.inventory.food.quantity += 1,
                    GuildItemNames::Fish => user.inventory.fish.quantity += 1,
                    GuildItemNames::Wood => user.inventory.wood.quantity += 1,
                    GuildItemNames::Ingots => user.inventory.ingots.quantity += 1,
                    GuildItemNames::Ore => user.inventory.ore.quantity += 1,
                },
                None => {}
            }

            match decrease_item {
                Some(item) => match item {
                    GuildItemNames::CookedFish => {
                        if user.inventory.food.quantity == 0 {
                            too_low_items(user, "cooked fish");
                        }

                        user.inventory.food.quantity -= 1;
                    }
                    GuildItemNames::Fish => {
                        if user.inventory.fish.quantity == 0 {
                            too_low_items(user, "fish");
                        }

                        user.inventory.fish.quantity -= 1;
                    }
                    GuildItemNames::Wood => {
                        if user.inventory.wood.quantity == 0 {
                            too_low_items(user, "wood");
                        }

                        user.inventory.wood.quantity -= 1;
                    }
                    GuildItemNames::Ingots => {
                        if user.inventory.ingots.quantity == 0 {
                            too_low_items(user, "ingots");
                        }

                        user.inventory.ingots.quantity -= 1;
                    }
                    GuildItemNames::Ore => {
                        if user.inventory.ore.quantity == 0 {
                            too_low_items(user, "ores");
                        }

                        user.inventory.ore.quantity -= 1;
                    }
                },
                None => {}
            }
        }
        1 => main_menu(user),
        _ => panic!("Dialogue selector picked invalid index."),
    }

    job(
        user,
        job_name.clone(),
        xp_type,
        use_gold,
        increase_item,
        decrease_item,
    );
}

fn too_low_items(user: &mut UserProfile, item_name: &str) {
    println!("\nYou do not have enough {} to work with.", item_name);
    tui::press_enter_to_continue();
    main_menu(user);
}
