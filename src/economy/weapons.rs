use crate::economy::shop::Item;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Weapon {
    WoodenSword,
    BronzeSword,
    IronSword,
    SteelSword,
    MysticSword,
    WizardStaff,
    InvalidItem,
}

pub mod shop {
    use super::{Item, Weapon};
    use std::collections::BTreeMap;

    use crate::{
        data::player::Player,
        utils::{
            input::select_from_vector,
            tui::{checkmark, table_from_csv},
        },
    };

    pub fn shop_list() -> BTreeMap<Weapon, Item> {
        BTreeMap::from([
            (Weapon::WoodenSword, Item::new("Wooden Sword", 10)),
            (Weapon::BronzeSword, Item::new("Bronze Sword", 50)),
            (Weapon::IronSword, Item::new("Iron Sword", 100)),
            (Weapon::SteelSword, Item::new("Steel Rapier", 500)),
            (Weapon::MysticSword, Item::new("Mystic Sword", 1_000)),
            (Weapon::WizardStaff, Item::new("Wizard Staff", 10_000)),
        ])
    }

    pub fn table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owned".to_string()];

        for (flag, item) in shop_list() {
            let owned = owns(player, flag);

            if owned.is_none() {
                panic!("Don't use the InvalidItem variant in the shop hash map.");
            }

            let string = format!(
                "{},{},{}",
                item.name,
                item.price,
                checkmark(*owned.expect("Should be a mutable usize ref"))
            );
            strings.push(string)
        }

        table_from_csv(strings);
    }

    pub fn build_transaction() -> Weapon {
        let shop = shop_list();
        let items = shop.values();
        let item_names: Vec<String> = items.map(|item| item.name.to_string()).collect();

        let selector = select_from_vector(item_names.clone(), None);
        let selected_item = item_names
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        let item = *shop_list()
            .iter()
            .find(|item| item.1.name == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag");

        item
    }

    pub fn get_item() -> Weapon {
        let shop = shop_list();
        let items = shop.values();
        let item_names: Vec<String> = items.map(|item| item.name.to_string()).collect();

        let selector = select_from_vector(item_names.clone(), None);
        let selected_item = item_names
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        *shop_list()
            .iter()
            .find(|item| item.1.name == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
    }

    pub fn owns(player: &mut Player, flag: Weapon) -> Option<&mut bool> {
        let item: Option<&mut bool> = match flag {
            Weapon::BronzeSword => Some(&mut player.weapons.bronze_sword.owns),
            Weapon::InvalidItem => None,
            Weapon::IronSword => Some(&mut player.weapons.iron_sword.owns),
            Weapon::MysticSword => Some(&mut player.weapons.mystic_sword.owns),
            Weapon::SteelSword => Some(&mut player.weapons.steel_sword.owns),
            Weapon::WizardStaff => Some(&mut player.weapons.wizard_staff.owns),
            Weapon::WoodenSword => Some(&mut player.weapons.wooden_sword.owns),
        };

        item
    }

    pub fn purchase(player: &mut Player, flag: Weapon, use_wallet: bool) -> Result<(), &str> {
        let shop = shop_list();
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

        let owns_item = owns(player, flag).expect("Don't use the InvalidItem variant in the shop hash map.");

        if *owns_item {
            return Err("You already own this item.");
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: Weapon, use_wallet: bool) -> Result<(), String> {
        let shop: BTreeMap<Weapon, Item> = shop_list();
        let shop_item: &Item = shop.get(&flag).expect("Item not found in hashmap.");
        let owns_item: &mut bool = owns(player, flag).expect("Don't use the InvalidItem variant in the shop hash map.");

        if !*owns_item {
            return Err("You do not own this item.".to_string());
        }

        *owns_item = false;

        if use_wallet {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = shop_item.price / 2;

            *wallet += price;
        }

        Ok(())
    }
}
