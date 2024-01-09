use serde::{Deserialize, Serialize};

use super::profile::UserProfile;

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMemberships {
    pub fishing: Guild,
    pub woodcutting: Guild,
    pub mining: Guild,
    pub smithing: Guild,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub member: bool,
    pub member_price: u32,
}

pub enum MemberGuilds {
    Fishing,
    Woodcutting,
    Mining,
    Smithing,
}

pub enum MembershipResult {
    Success,
    Failure(&'static str),
}

impl GuildMemberships {
    pub fn signup(user: &mut UserProfile, guild_flag: MemberGuilds) -> MembershipResult {
        let guild: &mut Guild = match guild_flag {
            MemberGuilds::Fishing => &mut user.guild_memberships.fishing,
            MemberGuilds::Woodcutting => &mut user.guild_memberships.woodcutting,
            MemberGuilds::Mining => &mut user.guild_memberships.mining,
            MemberGuilds::Smithing => &mut user.guild_memberships.smithing,
        };

        if user.gold < guild.member_price {
            return MembershipResult::Failure("You do not have enough gold.");
        }

        user.gold -= guild.member_price;
        guild.member = true;

        MembershipResult::Success
    }
}
