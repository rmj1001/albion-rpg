use serde::{Deserialize, Serialize};

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
}
