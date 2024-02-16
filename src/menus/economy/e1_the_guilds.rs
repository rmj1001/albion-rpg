#![allow(unused_assignments, unused_variables, unused_mut)]
use crate::utils::{
    input::{confirm, select_from_str_array},
    messages::*,
};

use crate::economy::items::{GuildItemNames, Item};

use crate::data::{
    guilds::{GuildMemberships, PricedGuilds},
    player::Player,
    xp::{XPType, XP},
};
use crate::utils::tui::page_header;
use crate::utils::tui::HeaderSubtext;
use rand::Rng;

pub fn main(player: &mut Player) {
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
        0 => check_membership(player, PricedGuilds::Fishing),
        1 => check_membership(player, PricedGuilds::Cooking),
        2 => check_membership(player, PricedGuilds::Woodcutting),
        3 => check_membership(player, PricedGuilds::Mining),
        4 => check_membership(player, PricedGuilds::Smithing),
        _ => {}
    }

    match guild_choice {
        0 => job(
            player,
            "Fishing".to_string(),
            XPType::Fishing,
            false,
            &Some(GuildItemNames::Fish),
            &None,
        ),
        1 => job(
            player,
            "Cooking".to_string(),
            XPType::Cooking,
            false,
            &Some(GuildItemNames::CookedFish),
            &Some(GuildItemNames::Fish),
        ),
        2 => job(
            player,
            "Woodcutting".to_string(),
            XPType::Woodcutting,
            false,
            &Some(GuildItemNames::Wood),
            &None,
        ),
        3 => job(
            player,
            "Mining".to_string(),
            XPType::Mining,
            false,
            &Some(GuildItemNames::Ore),
            &None,
        ),
        4 => job(
            player,
            "Smithing".to_string(),
            XPType::Smithing,
            false,
            &Some(GuildItemNames::Ingots),
            &Some(GuildItemNames::Ore),
        ),
        5 => job(player, "Thieving".to_string(), XPType::Thieving, true, &None, &None),
        6 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn check_membership(player: &mut Player, job: PricedGuilds) {
    let guild = match job {
        PricedGuilds::Fishing => &player.guild_memberships.fishing,
        PricedGuilds::Cooking => &player.guild_memberships.cooking,
        PricedGuilds::Woodcutting => &player.guild_memberships.woodcutting,
        PricedGuilds::Mining => &player.guild_memberships.mining,
        PricedGuilds::Smithing => &player.guild_memberships.smithing,
    };

    if guild.member {
        return;
    }

    if player.bank.wallet < guild.price {
        failure(format!(
            "This guild requires a membership, which you are too poor to purchase. (Cost: {} gold)",
            guild.price
        ));

        return main(player);
    }

    let permission_to_purchase = confirm(&format!(
        "This guild requires a membership (Cost: {} gold). Purchase?",
        guild.price
    ));

    if permission_to_purchase {
        if let Err(message) = GuildMemberships::purchase(player, job) {
            failure(message);
        }

        success_msg("You purchased the membership.");

        main(player);
    } else {
        failure("You cannot work in this guild without a membership.");
        main(player);
    }
}

fn job(
    player: &mut Player,
    job_name: String,
    xp_type: XPType,
    use_gold: bool,
    increase_item: &Option<GuildItemNames>,
    decrease_item: &Option<GuildItemNames>,
) {
    page_header(format!("Job: {}", job_name), HeaderSubtext::Keyboard);

    let xp = player.xp.get(xp_type);

    println!("XP: {}", xp);
    println!("Level: {}", XP::get_level(xp));

    if use_gold {
        println!();
        println!("Gold: {}", player.bank.wallet);
    }

    println!();

    match increase_item {
        Some(item) => match item {
            GuildItemNames::CookedFish => {
                println!("Cooked Fish: {}", player.items.food)
            }
            GuildItemNames::Fish => {
                println!("Fish: {}", player.items.fish)
            }
            GuildItemNames::Wood => {
                println!("Wood: {}", player.items.wood)
            }
            GuildItemNames::Ingots => {
                println!("Ingots: {}", player.items.ingots)
            }
            GuildItemNames::Ore => {
                println!("Ores: {}", player.items.ore)
            }
        },
        None => {}
    }

    match decrease_item {
        Some(item) => match item {
            GuildItemNames::CookedFish => {
                println!("Cooked Fish: {}", player.items.food)
            }
            GuildItemNames::Fish => {
                println!("Fish: {}", player.items.fish)
            }
            GuildItemNames::Wood => {
                println!("Wood: {}", player.items.wood)
            }
            GuildItemNames::Ingots => {
                println!("Ingots: {}", player.items.ingots)
            }
            GuildItemNames::Ore => {
                println!("Ores: {}", player.items.ore)
            }
        },
        None => {}
    }

    println!();

    let work_choice = select_from_str_array(&["Work", "NAV: Go Back"], None);

    match work_choice {
        0 => {
            player.xp.increment(xp_type);

            if use_gold {
                player.bank.wallet += rand::thread_rng().gen_range(0..2);

                job(
                    player,
                    job_name.clone(),
                    xp_type,
                    use_gold,
                    increase_item,
                    decrease_item,
                );
            }

            match increase_item {
                Some(item) => match item {
                    GuildItemNames::CookedFish => player.items.food += 1,
                    GuildItemNames::Fish => player.items.fish += 1,
                    GuildItemNames::Wood => player.items.wood += 1,
                    GuildItemNames::Ingots => player.items.ingots += 1,
                    GuildItemNames::Ore => player.items.ore += 1,
                },
                None => {}
            }

            match decrease_item {
                Some(item) => match item {
                    GuildItemNames::CookedFish => {
                        if player.items.food == 0 {
                            too_low_items(player, "cooked fish");
                        }

                        player.items.food -= 1;
                    }
                    GuildItemNames::Fish => {
                        if player.items.fish == 0 {
                            too_low_items(player, "fish");
                        }

                        player.items.fish -= 1;
                    }
                    GuildItemNames::Wood => {
                        if player.items.wood == 0 {
                            too_low_items(player, "wood");
                        }

                        player.items.wood -= 1;
                    }
                    GuildItemNames::Ingots => {
                        if player.items.ingots == 0 {
                            too_low_items(player, "ingots");
                        }

                        player.items.ingots -= 1;
                    }
                    GuildItemNames::Ore => {
                        if player.items.ore == 0 {
                            too_low_items(player, "ores");
                        }

                        player.items.ore -= 1;
                    }
                },
                None => {}
            }
        }
        1 => main(player),
        _ => out_of_bounds(),
    }

    job(
        player,
        job_name.clone(),
        xp_type,
        use_gold,
        increase_item,
        decrease_item,
    );
}

fn too_low_items(player: &mut Player, item_name: &str) {
    failure(format!("You do not have enough {} to work with.", item_name));
    main(player);
}
