#![allow(unused_assignments, unused_variables, unused_mut)]
use crate::{
    data::{
        guilds::{Guild, Guilds},
        inventory::items,
        xp::{XPType, XP},
    },
    prelude::{error, failure, page_header, random_num, select, success, unreachable, Instructions},
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("The Guilds", &Instructions::Keyboard);

    let guild_choice = select(
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
        0 => deter_non_members(player, Guild::Fishing),
        1 => deter_non_members(player, Guild::Cooking),
        2 => deter_non_members(player, Guild::Woodcutting),
        3 => deter_non_members(player, Guild::Mining),
        4 => deter_non_members(player, Guild::Smithing),
        5 => deter_non_members(player, Guild::Thieving),
        _ => {}
    }

    match guild_choice {
        0 => guild_menu(player, "Fishing", XPType::Fishing, items::GuildTypes::Fish, None),
        1 => guild_menu(
            player,
            "Cooking",
            XPType::Cooking,
            items::GuildTypes::Food,
            Some(items::GuildTypes::Fish),
        ),
        2 => guild_menu(
            player,
            "Woodcutting",
            XPType::Woodcutting,
            items::GuildTypes::Wood,
            None,
        ),
        3 => guild_menu(player, "Mining", XPType::Mining, items::GuildTypes::Ore, None),
        4 => guild_menu(
            player,
            "Smithing",
            XPType::Smithing,
            items::GuildTypes::Ingots,
            Some(items::GuildTypes::Ore),
        ),
        5 => guild_menu(player, "Thieving", XPType::Thieving, items::GuildTypes::Gold, None),
        6 => guild_membership_shop(player),
        7 => crate::menus::game_menu::main(player),
        _ => unreachable(),
    }
}

fn deter_non_members(player: &mut Player, guild: Guild) {
    if !player.guilds.check(guild) {
        failure("This guild requires a membership.\nPlease purchase one from the Memberships Office.\n");
        main(player);
    }
}

fn guild_menu(
    player: &mut Player,
    guild: &str,
    xp_type: XPType,
    increase_item: items::GuildTypes,
    decrease_item: Option<items::GuildTypes>,
) {
    print_guild_information(guild, player, xp_type, increase_item, decrease_item);
    let work_choice = select(&["Work", "NAV: Go Back"], None);

    match work_choice {
        0 => {
            if let Some(item) = decrease_item {
                let result: error::Result<()> = match item {
                    items::GuildTypes::Gold => {
                        let rand = random_num(1, 3);

                        if player.bank.wallet < rand {
                            player.bank.wallet = 0;
                        } else {
                            player.bank.wallet -= rand;
                        }
                        Ok(())
                    }
                    items::GuildTypes::Bait => try_subtract(&mut player.items.bait, "Bait"),
                    items::GuildTypes::Food => try_subtract(&mut player.items.food, "Cooked Fish"),
                    items::GuildTypes::Fish => try_subtract(&mut player.items.fish, "Fish"),
                    items::GuildTypes::Wood => try_subtract(&mut player.items.wood, "Wood"),
                    items::GuildTypes::Ingots => try_subtract(&mut player.items.ingots, "Ingots"),
                    items::GuildTypes::Ore => try_subtract(&mut player.items.ore, "Ore"),
                };

                if let Err(error) = result {
                    error.print(true);
                    guild_menu(player, guild, xp_type, increase_item, decrease_item);
                }
            }

            match increase_item {
                items::GuildTypes::Gold => player.bank.wallet += random_num(0, 2),
                items::GuildTypes::Bait => player.items.bait += 1,
                items::GuildTypes::Food => player.items.food += 1,
                items::GuildTypes::Fish => player.items.fish += 1,
                items::GuildTypes::Wood => player.items.wood += 1,
                items::GuildTypes::Ingots => player.items.ingots += 1,
                items::GuildTypes::Ore => player.items.ore += 1,
            }

            player.xp.increment(xp_type);
        }
        1 => main(player),
        _ => unreachable(),
    }

    guild_menu(player, guild, xp_type, increase_item, decrease_item);
}

fn print_guild_information(
    name: &str,
    player: &mut Player,
    xp_type: XPType,
    increase_item: items::GuildTypes,
    decrease_item: Option<items::GuildTypes>,
) {
    page_header(format!("Guild: {name}"), &Instructions::Keyboard);

    let xp = player.xp.get(xp_type);

    println!("XP: {xp}");
    println!("Level: {}", XP::get_level(*xp));
    println!();

    print_item(player, Some(increase_item));
    print_item(player, decrease_item);
    println!();
}

fn print_item(player: &mut Player, item: Option<items::GuildTypes>) {
    if let Some(item) = item {
        match item {
            items::GuildTypes::Gold => {
                println!("Gold: {}", player.bank.wallet);
            }
            items::GuildTypes::Bait => {
                println!("Bait: {}", player.items.bait);
            }
            items::GuildTypes::Food => {
                println!("Cooked Fish: {}", player.items.food);
            }
            items::GuildTypes::Fish => {
                println!("Fish: {}", player.items.fish);
            }
            items::GuildTypes::Wood => {
                println!("Wood: {}", player.items.wood);
            }
            items::GuildTypes::Ingots => {
                println!("Ingots: {}", player.items.ingots);
            }
            items::GuildTypes::Ore => {
                println!("Ores: {}", player.items.ore);
            }
        }
    }
}

fn try_subtract(item: &mut usize, item_name: &str) -> error::Result<()> {
    if *item == 0 {
        return Err(Box::new(error::Inventory::NotEnoughItem(item_name.to_string())));
    }

    *item -= 1;
    Ok(())
}

fn guild_membership_shop(player: &mut Player) {
    page_header("Guild Memberships Office", &Instructions::Keyboard);

    Guilds::shop_table(player);

    let choices = select(&["1. Join Guild", "2. Leave Guild", "3. Go Back"], None);

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
        _ => unreachable(),
    }
}

fn join_guild(player: &mut Player) {
    let flag = Guilds::select();
    match Guilds::join(player, flag, true) {
        Ok(()) => {
            success(None);
            guild_membership_shop(player);
        }
        Err(message) => {
            message.print(true);
            guild_membership_shop(player);
        }
    }
}

fn leave_guild(player: &mut Player) {
    let flag = Guilds::select();
    match Guilds::leave(player, flag, true) {
        Ok(()) => {
            success(None);
            guild_membership_shop(player);
        }
        Err(message) => {
            message.print(true);
            guild_membership_shop(player);
        }
    }
}
