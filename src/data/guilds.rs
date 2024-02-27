use crate::{
    data::player::Player,
    utils::{
        input::select_from_vector,
        tui::{checkmark, table_from_csv},
    },
    InventoryError, MiscError,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Guild {
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
    Thieving,
}

impl Guild {
    pub fn name(&self) -> &'static str {
        match self {
            Guild::Fishing => "Fishing",
            Guild::Cooking => "Cooking",
            Guild::Mining => "Mining",
            Guild::Woodcutting => "Woodcutting",
            Guild::Smithing => "Smithing",
            Guild::Thieving => "Thieving",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guilds {
    pub thieving: bool,
    pub fishing: bool,
    pub cooking: bool,
    pub woodcutting: bool,
    pub mining: bool,
    pub smithing: bool,
}

impl Guilds {
    pub fn new() -> Self {
        Self {
            thieving: false,
            fishing: false,
            cooking: false,
            woodcutting: false,
            mining: false,
            smithing: false,
        }
    }

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

        for (flag, price) in Self::shop().iter() {
            let string = format!("{},{},{}", flag.name(), price, checkmark(*player.guilds.get(flag)));
            strings.push(string)
        }

        table_from_csv(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn select() -> Guild {
        let shop: BTreeMap<Guild, usize> = Self::shop();
        let guilds: Vec<String> = shop.keys().map(|guild| guild.name().to_string()).collect();

        let selector: usize = select_from_vector(guilds.clone(), None);
        let selected_guild: String = guilds
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        let item: Guild = *Self::shop()
            .iter()
            .find(|guild| guild.0.name() == selected_guild)
            .map(|guild| guild.0)
            .expect("Should return a Guild flag");

        item
    }

    pub fn join(player: &mut Player, guild: Guild, payment: bool) -> crate::Result<()> {
        let shop: BTreeMap<Guild, usize> = Self::shop();
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

    pub fn leave(player: &mut Player, guild: Guild, payment: bool) -> crate::Result<()> {
        let shop: BTreeMap<Guild, usize> = Self::shop();
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
}
