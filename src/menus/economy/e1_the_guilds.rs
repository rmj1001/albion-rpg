#![allow(unused_assignments, unused_variables, unused_mut)]
use crate::{
    data::{
        guilds::{Guild, Guilds},
        inventory::items::GuildItem,
        xp::{XPType, XP},
    },
    prelude::*,
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("The Guilds", Instructions::Keyboard);

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
        0 => guild_menu(player, "Fishing", XPType::Fishing, GuildItem::Fish, None),
        1 => guild_menu(
            player,
            "Cooking",
            XPType::Cooking,
            GuildItem::Food,
            Some(GuildItem::Fish),
        ),
        2 => guild_menu(player, "Woodcutting", XPType::Woodcutting, GuildItem::Wood, None),
        3 => guild_menu(player, "Mining", XPType::Mining, GuildItem::Ore, None),
        4 => guild_menu(
            player,
            "Smithing",
            XPType::Smithing,
            GuildItem::Ingots,
            Some(GuildItem::Ore),
        ),
        5 => guild_menu(player, "Thieving", XPType::Thieving, GuildItem::Gold, None),
        6 => guild_membership_shop(player),
        7 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
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
    increase_item: GuildItem,
    decrease_item: Option<GuildItem>,
) {
    print_guild_information(guild, player, xp_type, increase_item, &decrease_item);
    let work_choice = select(&["Work", "NAV: Go Back"], None);

    match work_choice {
        0 => {
            if let Some(item) = decrease_item {
                let result: Result<()> = match item {
                    GuildItem::Gold => {
                        let rand = random_num(1, 3);

                        if player.bank.wallet < rand {
                            player.bank.wallet = 0;
                        } else {
                            player.bank.wallet -= rand;
                        }
                        Ok(())
                    }
                    GuildItem::Bait => try_subtract(&mut player.items.bait, "Bait"),
                    GuildItem::Food => try_subtract(&mut player.items.food, "Cooked Fish"),
                    GuildItem::Fish => try_subtract(&mut player.items.fish, "Fish"),
                    GuildItem::Wood => try_subtract(&mut player.items.wood, "Wood"),
                    GuildItem::Ingots => try_subtract(&mut player.items.ingots, "Ingots"),
                    GuildItem::Ore => try_subtract(&mut player.items.ore, "Ore"),
                };

                if let Err(error) = result {
                    error.print(true);
                    guild_menu(player, guild, xp_type, increase_item, decrease_item);
                }
            }

            match increase_item {
                GuildItem::Gold => player.bank.wallet += random_num(0, 2),
                GuildItem::Bait => player.items.bait += 1,
                GuildItem::Food => player.items.food += 1,
                GuildItem::Fish => player.items.fish += 1,
                GuildItem::Wood => player.items.wood += 1,
                GuildItem::Ingots => player.items.ingots += 1,
                GuildItem::Ore => player.items.ore += 1,
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
    increase_item: GuildItem,
    decrease_item: &Option<GuildItem>,
) {
    page_header(format!("Guild: {}", name), Instructions::Keyboard);

    let xp = player.xp.get(xp_type);

    println!("XP: {}", xp);
    println!("Level: {}", XP::get_level(*xp));
    println!();

    print_item(player, &Some(increase_item));
    print_item(player, decrease_item);
    println!();
}

fn print_item(player: &mut Player, item: &Option<GuildItem>) {
    match item {
        Some(item) => match item {
            GuildItem::Gold => {
                println!("Gold: {}", player.bank.wallet)
            }
            GuildItem::Bait => {
                println!("Bait: {}", player.items.bait)
            }
            GuildItem::Food => {
                println!("Cooked Fish: {}", player.items.food)
            }
            GuildItem::Fish => {
                println!("Fish: {}", player.items.fish)
            }
            GuildItem::Wood => {
                println!("Wood: {}", player.items.wood)
            }
            GuildItem::Ingots => {
                println!("Ingots: {}", player.items.ingots)
            }
            GuildItem::Ore => {
                println!("Ores: {}", player.items.ore)
            }
        },
        None => {}
    }
}

fn try_subtract(item: &mut usize, item_name: &str) -> Result<()> {
    if *item == 0 {
        return Err(Box::new(InventoryError::NotEnoughItem(item_name.to_string())));
    }

    *item -= 1;
    Ok(())
}

fn guild_membership_shop(player: &mut Player) {
    page_header("Guild Memberships Office", Instructions::Keyboard);

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
        _ => out_of_bounds(),
    }
}

fn join_guild(player: &mut Player) {
    let flag = Guilds::select();
    match Guilds::join(player, flag, true) {
        Ok(_) => {
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
        Ok(_) => {
            success(None);
            guild_membership_shop(player);
        }
        Err(message) => {
            message.print(true);
            guild_membership_shop(player);
        }
    }
}
