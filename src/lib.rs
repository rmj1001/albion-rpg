#![allow(unused_assignments, clippy::new_without_default)]

/*!
# ⚔️ Albion 🖥️ Terminal 🖥️ RPG ⚔️

A text-based RPG for the Terminal, written in Rust.

## Instructions

```sh
# Install
cargo install --locked albion_terminal_rpg
```

```sh
# Run
albionrpg
```

## 🕹️ Features

1. ⚔️ Elegant Combat System
    - 🗺️ Random Encounters ("Wander the Realm")
    - 💀 Endgame 50-battle Gauntlet ("The Stronghold")
    - 💊 Potions & Food for healing
2. 📈 Rich Economy
    - 🛍️ Elaborate Shops
        - ⚔️ Weapons
        - 🪖 Armor
        - 🐟 Mundane Items
    - 💰 Guilds for Income (Memberships Required)
        - 🐟 Fishing
        - 🍝 Cooking
        - 🪵 Woodcutting
        - ⛏️ Mining
        - 🔨 Smithing
        - 🚓 Thieving
3. ✅ XP & Achievements System
    - 💀 Monsters Killed
    - 💰 Earned 1,000,000 Gold
    - 💯 Level 100 (Player Total)
    - ⚔️ Stronghold Defeated
    - 💻 Hacked the Game

## 🖥️ Supported Platforms

- 🍎 MacOS
- 🪟 Windows
- 🐧 Linux
- 😈 BSD
*/

pub mod utils {
    pub mod crypt;
    pub mod error;
    pub mod files;
    pub mod input;
    pub mod math;
    pub mod messages;
    pub mod os;
    pub mod terminal;
    pub mod tui;
}

pub mod data {
    pub mod inventory {
        pub mod armor;
        pub mod bank;
        pub mod equipment;
        pub mod items;
        pub mod weapons;
    }
    pub mod achievements;
    pub mod guilds;
    pub mod health;
    pub mod player;
    pub mod settings;
    pub mod xp;
}

pub mod menus {
    pub mod devmode {
        pub mod d1_developer_menu;
        pub mod d2_user_mgr;
        pub mod d3_xp_mgr;
        pub mod d4_inventory_mgr;
        pub mod d5_bank_mgr;
    }

    pub mod economy {
        pub mod e1_the_guilds;
        pub mod e2_the_bank;
        pub mod e3_trading_post;
        pub mod e4_weapons_shop;
        pub mod e5_armor_shop;
    }

    pub mod profile {
        pub mod n1_settings;
        pub mod p1_inventory;
        pub mod p2_hall_of_records;
    }

    pub mod accounts;
    pub mod game_menu;
    pub mod login;
    pub mod register;
}

pub mod combat {

    pub mod battle;
    pub mod enemy;
    pub mod inventory;
}

pub mod prelude {
    pub use crate::utils::files::{handler as file_handler, *};
    pub use crate::utils::{crypt::*, error::*, input::*, math::*, messages::*, os::*, terminal::*, tui::*};
}
