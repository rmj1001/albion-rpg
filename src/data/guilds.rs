use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Membership {
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
    Thieving,
}

impl Membership {
    pub fn name(&self) -> &'static str {
        match self {
            Membership::Fishing => "Fishing",
            Membership::Cooking => "Cooking",
            Membership::Mining => "Mining",
            Membership::Woodcutting => "Woodcutting",
            Membership::Smithing => "Smithing",
            Membership::Thieving => "Thieving",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memberships {
    pub thieving: bool,
    pub fishing: bool,
    pub cooking: bool,
    pub woodcutting: bool,
    pub mining: bool,
    pub smithing: bool,
}

impl Memberships {
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

    pub fn get<'a>(&'a mut self, guild: &Membership) -> &'a mut bool {
        match guild {
            Membership::Thieving => &mut self.thieving,
            Membership::Cooking => &mut self.cooking,
            Membership::Fishing => &mut self.fishing,
            Membership::Mining => &mut self.mining,
            Membership::Smithing => &mut self.smithing,
            Membership::Woodcutting => &mut self.woodcutting,
        }
    }

    pub fn check(&mut self, guild: Membership) -> bool {
        match guild {
            Membership::Thieving => self.thieving,
            Membership::Cooking => self.cooking,
            Membership::Fishing => self.fishing,
            Membership::Mining => self.mining,
            Membership::Smithing => self.smithing,
            Membership::Woodcutting => self.woodcutting,
        }
    }

    pub fn toggle(&mut self, guild: Membership) {
        let guild = self.get(&guild);
        *guild = !*guild;
    }
}
