use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Item {
    name: String,
    price: usize,
}

impl Item {
    pub fn new(name: &str, price: usize) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ArmorFlag {
    Leather,
    Bronze,
    Iron,
    Steel,
    DragonHide,
    Mystic,
    InvalidItem,
}

pub mod shop {
    use super::{ArmorFlag, Item};
    use std::collections::BTreeMap;

    use crate::{
        data::player::Player,
        utils::{
            input::{input_generic, select_from_vector},
            tui::table_from_csv,
        },
    };

    pub fn shop_list() -> BTreeMap<ArmorFlag, Item> {
        BTreeMap::from([
            (ArmorFlag::Leather, Item::new("Leather", 100)),
            (ArmorFlag::Bronze, Item::new("Bronze", 300)),
            (ArmorFlag::Iron, Item::new("Iron", 1_000)),
            (ArmorFlag::Steel, Item::new("Steel", 5_000)),
            (ArmorFlag::DragonHide, Item::new("Dragon Hide", 10_000)),
            (ArmorFlag::Mystic, Item::new("Mystic", 20_000)),
        ])
    }

    pub fn table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price".to_string()];

        for (flag, item) in shop_list() {
            let quantity = owns(player, flag);

            if quantity.is_none() {
                panic!("Don't use the InvalidItem variant in the shop hash map.");
            }

            let string = format!(
                "{},{},{}",
                item.name,
                item.price,
                quantity.expect("Should be a mutable usize ref")
            );
            strings.push(string)
        }

        table_from_csv(strings);
    }

    pub fn build_transaction() -> (ArmorFlag, usize) {
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

        let quantity = input_generic::<usize>("Quantity:").expect("Did not get valid usize");

        (item, quantity)
    }

    pub fn get_item() -> ArmorFlag {
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

    pub fn owns(player: &mut Player, flag: ArmorFlag) -> Option<&mut bool> {
        let item: Option<&mut bool> = match flag {
            ArmorFlag::Bronze => Some(&mut player.armor.bronze.owns),
            ArmorFlag::DragonHide => Some(&mut player.armor.dragonhide.owns),
            ArmorFlag::InvalidItem => None,
            ArmorFlag::Iron => Some(&mut player.armor.iron.owns),
            ArmorFlag::Leather => Some(&mut player.armor.leather.owns),
            ArmorFlag::Mystic => Some(&mut player.armor.mystic.owns),
            ArmorFlag::Steel => Some(&mut player.armor.steel.owns),
        };

        item
    }

    pub fn purchase(player: &mut Player, flag: ArmorFlag, quantity: usize, use_wallet: bool) -> Result<(), &str> {
        let shop = shop_list();
        let item: &Item = shop.get(&flag).expect("Item not found in hashmap.");

        if use_wallet {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let price = quantity * item.price;

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

    pub fn sell(player: &mut Player, flag: ArmorFlag, quantity: usize, use_wallet: bool) -> Result<(), String> {
        let shop: BTreeMap<ArmorFlag, Item> = shop_list();
        let shop_item: &Item = shop.get(&flag).expect("Item not found in hashmap.");
        let owns_item: &mut bool = owns(player, flag).expect("Don't use the InvalidItem variant in the shop hash map.");

        if !*owns_item {
            return Err("You do not own this item.".to_string());
        }

        *owns_item = false;

        if use_wallet {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = quantity * (shop_item.price / 2);

            *wallet += price;
        }

        Ok(())
    }
}
