#![allow(unused_assignments, unused_variables, unused_mut)]
use crate::lib::{
    input::{confirm, select_from_str_array},
    messages::*,
};

use crate::lib::tui::page_header;
use crate::lib::tui::HeaderSubtext;
use crate::user::{
    guilds::{GuildMemberships, PricedGuilds},
    inventory::{GuildItemNames, Item},
    profile::UserProfile,
    xp::{XPType, XP},
};
use rand::Rng;

pub fn main(user: &mut UserProfile) {
    page_header("The Guilds", HeaderSubtext::Keyboard);

    let job_name: String;
    let job_xp: XPType;
    let use_gold: bool;
    let mut increase_item: Option<&mut Item>;
    let mut decrease_item: Option<&mut Item>;

    let guild_choice = select_from_str_array(
        &[
            "1. Fishing",
            "2. Cooking",
            "3. Woodcutting",
            "4. Mining",
            "5. Smithing",
            "6. Thieving",
            "NAV: Go Back",
        ],
        None,
    );

    match guild_choice {
        0 => check_membership(user, PricedGuilds::Fishing),
        1 => check_membership(user, PricedGuilds::Cooking),
        2 => check_membership(user, PricedGuilds::Woodcutting),
        3 => check_membership(user, PricedGuilds::Mining),
        4 => check_membership(user, PricedGuilds::Smithing),
        _ => {}
    }

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
        6 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(),
    }
}

fn check_membership(user: &mut UserProfile, job: PricedGuilds) {
    let guild = match job {
        PricedGuilds::Fishing => &user.guild_memberships.fishing,
        PricedGuilds::Cooking => &user.guild_memberships.cooking,
        PricedGuilds::Woodcutting => &user.guild_memberships.woodcutting,
        PricedGuilds::Mining => &user.guild_memberships.mining,
        PricedGuilds::Smithing => &user.guild_memberships.smithing,
    };

    if guild.member {
        return;
    }

    if user.bank.wallet < guild.member_price {
        failure(format!(
            "This guild requires a membership, which you are too poor to purchase. (Cost: {} gold)",
            guild.member_price
        ));

        return main(user);
    }

    let permission_to_purchase = confirm(&format!(
        "This guild requires a membership (Cost: {} gold). Purchase?",
        guild.member_price
    ));

    if permission_to_purchase {
        if let Err(message) = GuildMemberships::purchase(user, job) {
            failure(message);
        }

        main(user);
    } else {
        failure("You cannot work in this guild without a membership.");
        main(user);
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
    page_header(format!("Job: {}", job_name), HeaderSubtext::Keyboard);

    let xp = user.xp.get(xp_type);

    println!("XP: {}", xp);
    println!("Level: {}", XP::level(xp));

    if use_gold {
        println!();
        println!("Gold: {}", user.bank.wallet);
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

    let work_choice = select_from_str_array(&["Work", "Go Back"], None);

    match work_choice {
        0 => {
            user.xp.increment(xp_type);

            if use_gold {
                user.bank.wallet += rand::thread_rng().gen_range(0..2);

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
        1 => main(user),
        _ => out_of_bounds(),
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
    failure(format!(
        "You do not have enough {} to work with.",
        item_name
    ));
    main(user);
}
