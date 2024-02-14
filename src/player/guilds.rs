use serde::{Deserialize, Serialize};

use super::profile::Player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMemberships {
    pub fishing: Guild,
    pub cooking: Guild,
    pub woodcutting: Guild,
    pub mining: Guild,
    pub smithing: Guild,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub member: bool,
    pub member_price: usize,
}

pub enum PricedGuilds {
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
}

impl GuildMemberships {
    pub fn purchase(player: &mut Player, guild_flag: PricedGuilds) -> Result<(), &'static str> {
        let guild: &mut Guild = match guild_flag {
            PricedGuilds::Fishing => &mut player.guild_memberships.fishing,
            PricedGuilds::Cooking => &mut player.guild_memberships.cooking,
            PricedGuilds::Woodcutting => &mut player.guild_memberships.woodcutting,
            PricedGuilds::Mining => &mut player.guild_memberships.mining,
            PricedGuilds::Smithing => &mut player.guild_memberships.smithing,
        };

        if player.bank.wallet < guild.member_price {
            return Err("You do not have enough gold.");
        }

        player.bank.wallet -= guild.member_price;
        guild.member = true;
        Ok(())
    }
}
