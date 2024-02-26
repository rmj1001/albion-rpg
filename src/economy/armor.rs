use crate::data::inventory::armor::Armor;
use crate::economy::shop::Item;

pub mod shop {
    use super::{Armor, Item};
    use std::collections::BTreeMap;

    use crate::{
        data::player::Player,
        utils::{
            input::select_from_vector,
            tui::{checkmark, table_from_csv},
        },
        InventoryError,
    };

    pub fn list() -> BTreeMap<Armor, Item> {
        BTreeMap::from([
            (Armor::Leather, Item::new("Leather", 100)),
            (Armor::Bronze, Item::new("Bronze", 300)),
            (Armor::Iron, Item::new("Iron", 1_000)),
            (Armor::Steel, Item::new("Steel", 5_000)),
            (Armor::Dragonhide, Item::new("Dragon Hide", 10_000)),
            (Armor::Mystic, Item::new("Mystic", 20_000)),
        ])
    }

    pub fn table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owns".to_string()];

        for (flag, item) in list() {
            let owned = owns(player, &flag);

            let string = format!("{},{},{}", item.name, item.price, checkmark(*owned));
            strings.push(string)
        }

        table_from_csv(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn picker<'a>() -> Armor {
        let shop = list();
        let items = shop.values();
        let item_names: Vec<String> = items.map(|item| item.name.to_string()).collect();

        let selector = select_from_vector(item_names.clone(), None);
        let selected_item = item_names
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        list()
            .iter()
            .find(|item| item.1.name == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
            .clone()
    }

    pub fn owns<'a>(player: &'a mut Player, armor: &Armor) -> &'a mut bool {
        match armor {
            Armor::Bronze => &mut player.armor.bronze.owns,
            Armor::Dragonhide => &mut player.armor.dragonhide.owns,
            Armor::Iron => &mut player.armor.iron.owns,
            Armor::Leather => &mut player.armor.leather.owns,
            Armor::Mystic => &mut player.armor.mystic.owns,
            Armor::Steel => &mut player.armor.steel.owns,
        }
    }

    pub fn buy(player: &mut Player, armor: &Armor, payment: bool) -> crate::Result<()> {
        let shop = list();
        let item: &Item = shop.get(&armor).expect("Item not found in hashmap.");

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let price = item.price;

            if gold < price {
                return Err(InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= price;
        }

        let owns_item = owns(player, armor);

        if *owns_item {
            return Err(InventoryError::ItemOwned.boxed());
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, armor: &Armor, payment: bool) -> crate::Result<()> {
        let shop: BTreeMap<Armor, Item> = list();
        let shop_item: &Item = shop.get(&armor).expect("Item not found in hashmap.");
        let owns_item: &mut bool = owns(player, armor);

        if !*owns_item {
            return Err(InventoryError::ItemNotOwned.boxed());
        }

        *owns_item = false;

        if payment {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = shop_item.price / 2;

            *wallet += price;
        }

        Ok(())
    }
}
