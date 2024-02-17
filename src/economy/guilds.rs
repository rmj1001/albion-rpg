use crate::{
    data::player::Player,
    economy::shop::Item,
    utils::{
        input::select_from_vector,
        tui::{checkmark, table_from_csv},
    },
};
use std::collections::BTreeMap;

use super::items::Items;

#[derive(Clone, Copy, Debug)]
pub enum GuildItems {
    Bait,
    Fish,
    CookedFish,
    Wood,
    Ore,
    Ingots,
    Gold,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Membership {
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
    Thieving,
}

pub fn shop_list() -> BTreeMap<Membership, Item> {
    BTreeMap::from([
        (Membership::Thieving, Item::new("Thieving", 10)),
        (Membership::Fishing, Item::new("Fishing", 100)),
        (Membership::Cooking, Item::new("Cooking", 200)),
        (Membership::Woodcutting, Item::new("Woodcutting", 300)),
        (Membership::Mining, Item::new("Mining", 500)),
        (Membership::Smithing, Item::new("Smithing", 1_000)),
    ])
}

pub fn table(player: &mut Player) {
    let mut strings: Vec<String> = vec!["Guild,Price,Member".to_string()];

    for (flag, item) in shop_list().iter() {
        let string = format!(
            "{},{},{}",
            item.name,
            item.price,
            checkmark(check_membership(player, *flag))
        );
        strings.push(string)
    }

    table_from_csv(strings);
    println!("Gold: {}\n", player.bank.wallet);
}

pub fn get_membership(player: &mut Player, guild: Membership) -> &mut bool {
    match guild {
        Membership::Thieving => &mut player.guilds.thieving,
        Membership::Cooking => &mut player.guilds.cooking,
        Membership::Fishing => &mut player.guilds.fishing,
        Membership::Mining => &mut player.guilds.mining,
        Membership::Smithing => &mut player.guilds.smithing,
        Membership::Woodcutting => &mut player.guilds.woodcutting,
    }
}

pub fn check_membership(player: &mut Player, guild: Membership) -> bool {
    match guild {
        Membership::Thieving => player.guilds.thieving,
        Membership::Cooking => player.guilds.cooking,
        Membership::Fishing => player.guilds.fishing,
        Membership::Mining => player.guilds.mining,
        Membership::Smithing => player.guilds.smithing,
        Membership::Woodcutting => player.guilds.woodcutting,
    }
}

pub fn toggle_membership(player: &mut Player, guild: Membership) {
    let guilds = &mut player.guilds;

    match guild {
        Membership::Thieving => guilds.thieving = !guilds.thieving,
        Membership::Cooking => guilds.cooking = !guilds.cooking,
        Membership::Fishing => guilds.fishing = !guilds.fishing,
        Membership::Mining => guilds.mining = !guilds.mining,
        Membership::Smithing => guilds.smithing = !guilds.smithing,
        Membership::Woodcutting => guilds.woodcutting = !guilds.woodcutting,
    }
}

pub fn build_transaction() -> Membership {
    let shop: BTreeMap<Membership, Item> = shop_list();
    let guilds = shop.values();
    let guild_names: Vec<String> = guilds.map(|guild| guild.name.to_string()).collect();

    let selector: usize = select_from_vector(guild_names.clone(), None);
    let selected_guild: String = guild_names
        .get(selector)
        .expect("This shouldn't select a vector item out of bounds.")
        .to_string();

    let item: Membership = *shop_list()
        .iter()
        .find(|guild| guild.1.name == selected_guild)
        .map(|guild| guild.0)
        .expect("Should return a Guild flag");

    item
}

// Bug: Broken?
pub fn purchase(player: &mut Player, flag: Membership, use_wallet: bool) -> Result<(), &str> {
    let shop: BTreeMap<Membership, Item> = shop_list();
    let item: &Item = shop.get(&flag).expect("Item not found in hashmap.");

    if use_wallet {
        let gold: usize = player.bank.wallet;
        let wallet: &mut usize = &mut player.bank.wallet;
        let price = item.price;

        if gold < price {
            return Err("Not enough gold.");
        }

        *wallet -= price;
    }

    if check_membership(player, flag) {
        return Err("You are already a member of this guild.");
    }

    toggle_membership(player, flag);

    Ok(())
}

pub fn sell(player: &mut Player, flag: Membership, use_wallet: bool) -> Result<(), String> {
    let shop: BTreeMap<Membership, Item> = shop_list();
    let shop_item: &Item = shop.get(&flag).expect("Item not found in hashmap.");

    if use_wallet {
        let wallet: &mut usize = &mut player.bank.wallet;
        let price: usize = shop_item.price / 2;

        *wallet += price;
    }

    if !check_membership(player, flag) {
        return Err("You are not a member of this guild.".to_string());
    }

    toggle_membership(player, flag);

    Ok(())
}

pub fn guild_item_to_item(flag: GuildItems) -> Option<Items> {
    match flag {
        GuildItems::Ore => Some(Items::Ore),
        GuildItems::Bait => Some(Items::Bait),
        GuildItems::Fish => Some(Items::Fish),
        GuildItems::CookedFish => Some(Items::Food),
        GuildItems::Ingots => Some(Items::Ingots),
        GuildItems::Wood => Some(Items::Wood),
        GuildItems::Gold => None,
    }
}
