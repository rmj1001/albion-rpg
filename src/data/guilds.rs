use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMemberships {
    pub fishing: Guild,
    pub cooking: Guild,
    pub woodcutting: Guild,
    pub mining: Guild,
    pub smithing: Guild,
}

impl GuildMemberships {
    pub fn new() -> Self {
        Self {
            fishing: Guild::new(100),
            cooking: Guild::new(150),
            woodcutting: Guild::new(300),
            mining: Guild::new(500),
            smithing: Guild::new(1_000),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn purchase(player: &mut Player, guild_flag: PricedGuilds) -> Result<(), &'static str> {
        let guild: &mut Guild = match guild_flag {
            PricedGuilds::Fishing => &mut player.guild_memberships.fishing,
            PricedGuilds::Cooking => &mut player.guild_memberships.cooking,
            PricedGuilds::Woodcutting => &mut player.guild_memberships.woodcutting,
            PricedGuilds::Mining => &mut player.guild_memberships.mining,
            PricedGuilds::Smithing => &mut player.guild_memberships.smithing,
        };

        if player.bank.wallet < guild.price {
            return Err("You do not have enough gold.");
        }

        player.bank.wallet -= guild.price;
        guild.member = true;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub member: bool,
    pub price: usize,
}

impl Guild {
    pub fn new(price: usize) -> Self {
        Self { member: false, price }
    }
}

pub enum PricedGuilds {
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
}
