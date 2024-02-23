#![allow(unused_assignments, unused_variables, unused_mut)]
use crate::{
    data::xp::{XPType, XP},
    economy::guilds::{self, check_membership, GuildItems, Membership},
    utils::{
        error::CustomError,
        input::select_from_str_array,
        math::random_num,
        messages::{failure, out_of_bounds, success},
        tui::{page_header, HeaderSubtext},
    },
    InventoryError,
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("The Guilds", HeaderSubtext::Keyboard);

    let guild_choice = select_from_str_array(
        &[
            "1. Guild: Fishing",
            "2. Guild: Cooking",
            "3. Guild: Woodcutting",
            "4. Guild: Mining",
            "5. Guild: Smithing",
            "6. Guild: Thieving",
            "7. Membership Shop",
            "NAV: Go Back",
        ],
        None,
    );

    match guild_choice {
        0 => deter_outsiders(player, Membership::Fishing),
        1 => deter_outsiders(player, Membership::Cooking),
        2 => deter_outsiders(player, Membership::Woodcutting),
        3 => deter_outsiders(player, Membership::Mining),
        4 => deter_outsiders(player, Membership::Smithing),
        5 => deter_outsiders(player, Membership::Thieving),
        _ => {}
    }

    match guild_choice {
        0 => guild_menu(player, "Fishing", XPType::Fishing, GuildItems::Fish, None),
        1 => guild_menu(
            player,
            "Cooking",
            XPType::Cooking,
            GuildItems::CookedFish,
            Some(GuildItems::Fish),
        ),
        2 => guild_menu(player, "Woodcutting", XPType::Woodcutting, GuildItems::Wood, None),
        3 => guild_menu(player, "Mining", XPType::Mining, GuildItems::Ore, None),
        4 => guild_menu(
            player,
            "Smithing",
            XPType::Smithing,
            GuildItems::Ingots,
            Some(GuildItems::Ore),
        ),
        5 => guild_menu(player, "Thieving", XPType::Thieving, GuildItems::Gold, None),
        6 => guild_membership_shop(player),
        7 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn deter_outsiders(player: &mut Player, guild: Membership) {
    if !check_membership(player, guild) {
        failure("This guild requires a membership.\nPlease purchase one from the Memberships Office.\n");
        main(player);
    }
}

fn guild_menu(
    player: &mut Player,
    guild: &str,
    xp_type: XPType,
    increase_item: GuildItems,
    decrease_item: Option<GuildItems>,
) {
    print_guild_information(guild, player, xp_type, increase_item, &decrease_item);
    let work_choice = select_from_str_array(&["Work", "NAV: Go Back"], None);

    match work_choice {
        0 => {
            if let Some(item) = decrease_item {
                let result: Result<(), InventoryError> = match item {
                    GuildItems::Gold => {
                        let rand = random_num(1, 3);

                        if player.bank.wallet < rand {
                            player.bank.wallet = 0;
                        } else {
                            player.bank.wallet -= rand;
                        }
                        Ok(())
                    }
                    GuildItems::Bait => try_subtract(&mut player.items.bait, "Bait"),
                    GuildItems::CookedFish => try_subtract(&mut player.items.food, "Cooked Fish"),
                    GuildItems::Fish => try_subtract(&mut player.items.fish, "Fish"),
                    GuildItems::Wood => try_subtract(&mut player.items.wood, "Wood"),
                    GuildItems::Ingots => try_subtract(&mut player.items.ingots, "Ingots"),
                    GuildItems::Ore => try_subtract(&mut player.items.ore, "Ore"),
                };

                if let Err(error) = result {
                    error.failure();
                    guild_menu(player, guild, xp_type, increase_item, decrease_item);
                }
            }

            match increase_item {
                GuildItems::Gold => player.bank.wallet += random_num(0, 2),
                GuildItems::Bait => player.items.bait += 1,
                GuildItems::CookedFish => player.items.food += 1,
                GuildItems::Fish => player.items.fish += 1,
                GuildItems::Wood => player.items.wood += 1,
                GuildItems::Ingots => player.items.ingots += 1,
                GuildItems::Ore => player.items.ore += 1,
            }

            player.xp.increment(xp_type);
        }
        1 => main(player),
        _ => out_of_bounds(),
    }

    guild_menu(player, guild, xp_type, increase_item, decrease_item);
}

fn print_guild_information(
    name: &str,
    player: &mut Player,
    xp_type: XPType,
    increase_item: GuildItems,
    decrease_item: &Option<GuildItems>,
) {
    page_header(format!("Guild: {}", name), HeaderSubtext::Keyboard);

    let xp = player.xp.get(xp_type);

    println!("XP: {}", xp);
    println!("Level: {}", XP::get_level(xp));
    println!();

    print_item(player, &Some(increase_item));
    print_item(player, decrease_item);
    println!();
}

fn print_item(player: &mut Player, item: &Option<GuildItems>) {
    match item {
        Some(item) => match item {
            GuildItems::Gold => {
                println!("Gold: {}", player.bank.wallet)
            }
            GuildItems::Bait => {
                println!("Bait: {}", player.items.bait)
            }
            GuildItems::CookedFish => {
                println!("Cooked Fish: {}", player.items.food)
            }
            GuildItems::Fish => {
                println!("Fish: {}", player.items.fish)
            }
            GuildItems::Wood => {
                println!("Wood: {}", player.items.wood)
            }
            GuildItems::Ingots => {
                println!("Ingots: {}", player.items.ingots)
            }
            GuildItems::Ore => {
                println!("Ores: {}", player.items.ore)
            }
        },
        None => {}
    }
}

fn try_subtract(item: &mut usize, item_name: &str) -> Result<(), InventoryError> {
    if *item == 0 {
        return Err(InventoryError::NotEnoughItem(item_name.to_string()));
    }

    *item -= 1;
    Ok(())
}

fn guild_membership_shop(player: &mut Player) {
    page_header("Guild Memberships Office", HeaderSubtext::Keyboard);

    guilds::table(player);

    let choices = select_from_str_array(&["1. Join Guild", "2. Leave Guild", "3. Go Back"], None);

    match choices {
        0 => {
            join_guild(player);
            guild_membership_shop(player);
        }
        1 => {
            leave_guild(player);
            guild_membership_shop(player);
        }
        2 => main(player),
        _ => out_of_bounds(),
    }
}

fn join_guild(player: &mut Player) {
    let flag = guilds::build_transaction();
    match guilds::purchase(player, flag, true) {
        Ok(_) => {
            success();
            guild_membership_shop(player);
        }
        Err(message) => {
            message.failure();
            guild_membership_shop(player);
        }
    }
}

fn leave_guild(player: &mut Player) {
    let flag = guilds::build_transaction();
    match guilds::sell(player, flag, true) {
        Ok(_) => {
            success();
            guild_membership_shop(player);
        }
        Err(message) => {
            message.failure();
            guild_membership_shop(player);
        }
    }
}
