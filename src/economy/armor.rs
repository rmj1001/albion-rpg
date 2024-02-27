use crate::data::inventory::armor::Armor;

pub mod shop {
    use super::Armor;
    use std::collections::BTreeMap;

    use crate::{
        data::player::Player,
        utils::{
            input::select_from_vector,
            tui::{checkmark, table_from_csv},
        },
        InventoryError,
    };

    pub fn list() -> BTreeMap<Armor, usize> {
        BTreeMap::from([
            (Armor::Leather, 100),
            (Armor::Bronze, 300),
            (Armor::Iron, 1_000),
            (Armor::Steel, 5_000),
            (Armor::Dragonhide, 10_000),
            (Armor::Mystic, 20_000),
        ])
    }

    pub fn table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owns".to_string()];

        for (flag, price) in list() {
            let string = format!("{},{},{}", flag.name(), price, checkmark(player.armor.get(&flag).owns));
            strings.push(string)
        }

        table_from_csv(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn picker() -> Armor {
        let shop = list();
        let items: Vec<String> = shop.keys().map(|flag| flag.name().to_string()).collect();

        let selector = select_from_vector(items.clone(), None);
        let selected_item = items
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        list()
            .iter()
            .find(|item| item.0.name() == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
            .clone()
    }

    pub fn buy(player: &mut Player, flag: &Armor, payment: bool) -> crate::Result<()> {
        let shop = list();
        let price: &usize = shop.get(flag).expect("Item not found in hashmap.");

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;

            if gold < *price {
                return Err(InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= *price;
        }

        let owns_item = &mut player.armor.get(flag).owns;

        if *owns_item {
            return Err(InventoryError::ItemOwned.boxed());
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: &Armor, payment: bool) -> crate::Result<()> {
        let shop: BTreeMap<Armor, usize> = list();
        let price: &usize = shop.get(flag).expect("Item not found in hashmap.");
        let owns_item: &mut bool = &mut player.armor.get(flag).owns;

        if !*owns_item {
            return Err(InventoryError::ItemNotOwned.boxed());
        }

        *owns_item = false;

        if payment {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = price / 2;

            *wallet += price;
        }

        Ok(())
    }
}
