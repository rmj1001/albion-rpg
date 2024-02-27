use crate::{
    data::{guilds::Membership, player::Player},
    utils::{
        input::select_from_vector,
        tui::{checkmark, table_from_csv},
    },
    InventoryError, MiscError,
};
use std::collections::BTreeMap;

pub fn list() -> BTreeMap<Membership, usize> {
    BTreeMap::from([
        (Membership::Thieving, 10),
        (Membership::Fishing, 100),
        (Membership::Cooking, 200),
        (Membership::Woodcutting, 300),
        (Membership::Mining, 500),
        (Membership::Smithing, 1_000),
    ])
}

pub fn table(player: &mut Player) {
    let mut strings: Vec<String> = vec!["Guild,Price,Member".to_string()];

    for (flag, price) in list().iter() {
        let string = format!("{},{},{}", flag.name(), price, checkmark(*player.guilds.get(flag)));
        strings.push(string)
    }

    table_from_csv(strings);
    println!("Gold: {}\n", player.bank.wallet);
}

pub fn picker() -> Membership {
    let shop: BTreeMap<Membership, usize> = list();
    let guilds: Vec<String> = shop.keys().map(|guild| guild.name().to_string()).collect();

    let selector: usize = select_from_vector(guilds.clone(), None);
    let selected_guild: String = guilds
        .get(selector)
        .expect("This shouldn't select a vector item out of bounds.")
        .to_string();

    let item: Membership = *list()
        .iter()
        .find(|guild| guild.0.name() == selected_guild)
        .map(|guild| guild.0)
        .expect("Should return a Guild flag");

    item
}

pub fn buy(player: &mut Player, guild: Membership, payment: bool) -> crate::Result<()> {
    let shop: BTreeMap<Membership, usize> = list();
    let price: &usize = shop.get(&guild).expect("Item not found in hashmap.");

    if player.guilds.check(guild) {
        return Err(MiscError::Custom("You are already a guild member.").boxed());
    }

    if payment {
        let gold: usize = player.bank.wallet;
        let wallet: &mut usize = &mut player.bank.wallet;

        if gold < *price {
            return Err(InventoryError::NotEnoughGold.boxed());
        }

        *wallet -= *price;
    }

    player.guilds.toggle(guild);

    Ok(())
}

pub fn sell(player: &mut Player, guild: Membership, payment: bool) -> crate::Result<()> {
    let shop: BTreeMap<Membership, usize> = list();
    let price: &usize = shop.get(&guild).expect("Item not found in hashmap.");

    if !player.guilds.check(guild) {
        return Err(MiscError::Custom("You not a member of this guild.").boxed());
    }

    if payment {
        let wallet: &mut usize = &mut player.bank.wallet;
        let price: usize = *price / 2;

        *wallet += price;
    }

    player.guilds.toggle(guild);

    Ok(())
}
