use crate::prelude::{checkmark, csv_table, error, select};
use crate::{data::player::Player, panic_menu};

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Guild {
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
    Thieving,
}

impl Display for Guild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Guild::Fishing => "Fishing",
                Guild::Cooking => "Cooking",
                Guild::Mining => "Mining",
                Guild::Woodcutting => "Woodcutting",
                Guild::Smithing => "Smithing",
                Guild::Thieving => "Thieving",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Guilds {
    pub thieving: bool,
    pub fishing: bool,
    pub cooking: bool,
    pub woodcutting: bool,
    pub mining: bool,
    pub smithing: bool,
}

impl Guilds {
    pub fn get<'a>(&'a mut self, guild: &Guild) -> &'a mut bool {
        match guild {
            Guild::Thieving => &mut self.thieving,
            Guild::Cooking => &mut self.cooking,
            Guild::Fishing => &mut self.fishing,
            Guild::Mining => &mut self.mining,
            Guild::Smithing => &mut self.smithing,
            Guild::Woodcutting => &mut self.woodcutting,
        }
    }

    pub fn check(&mut self, guild: Guild) -> bool {
        match guild {
            Guild::Thieving => self.thieving,
            Guild::Cooking => self.cooking,
            Guild::Fishing => self.fishing,
            Guild::Mining => self.mining,
            Guild::Smithing => self.smithing,
            Guild::Woodcutting => self.woodcutting,
        }
    }

    pub fn toggle(&mut self, guild: Guild) {
        let guild = self.get(&guild);
        *guild = !*guild;
    }
}

// -------------------------------------------------- Economy -------------------------------------------------- //

impl Guilds {
    fn shop() -> BTreeMap<Guild, usize> {
        BTreeMap::from([
            (Guild::Thieving, 10),
            (Guild::Fishing, 100),
            (Guild::Cooking, 200),
            (Guild::Woodcutting, 300),
            (Guild::Mining, 500),
            (Guild::Smithing, 1_000),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Guild,Price,Member".to_string()];

        for (flag, price) in &Self::shop() {
            strings.push(format!("{},{},{}", flag, price, checkmark(*player.guilds.get(flag))));
        }

        csv_table(&strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn select() -> Guild {
        let shop: BTreeMap<Guild, usize> = Self::shop();
        let guilds: Vec<String> = shop.keys().map(std::string::ToString::to_string).collect();

        let selector: usize = select(&guilds, None);
        let selected_guild: String = guilds
            .get(selector)
            .unwrap_or_else(|| panic_menu!("Selected a guild from the guild hashmap out of bounds"))
            .to_string();

        let item: Guild = *Self::shop()
            .iter()
            .find(|guild| guild.0.to_string() == selected_guild)
            .map_or_else(|| panic_menu!("Guild flag selected out of bounds"), |guild| guild.0);

        item
    }

    pub fn join(player: &mut Player, guild: Guild, payment: bool) -> error::Result<()> {
        let shop: BTreeMap<Guild, usize> = Self::shop();
        let price: &usize = if let Some(item) = shop.get(&guild) {
            item
        } else {
            panic_menu!("Guild membership flag not found in hashmap.");
        };

        if player.guilds.check(guild) {
            return Err(Box::new(error::Miscellaneous::Custom(
                "You are already a guild member.",
            )));
        }

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;

            if gold < *price {
                return Err(Box::new(error::Inventory::NotEnoughGold));
            }

            *wallet -= *price;
        }

        player.guilds.toggle(guild);

        Ok(())
    }

    pub fn leave(player: &mut Player, guild: Guild, payment: bool) -> error::Result<()> {
        let shop: BTreeMap<Guild, usize> = Self::shop();
        let price: &usize = if let Some(item) = shop.get(&guild) {
            item
        } else {
            panic_menu!("Guild membership flag not found in hashmap.");
        };

        if !player.guilds.check(guild) {
            return Err(Box::new(error::Miscellaneous::Custom(
                "You not a member of this guild.",
            )));
        }

        if payment {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = *price / 2;

            *wallet += price;
        }

        player.guilds.toggle(guild);

        Ok(())
    }
}
