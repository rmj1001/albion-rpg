use crate::data::inventory::weapons::Weapon;

pub mod shop {
    use super::Weapon;
    use std::collections::BTreeMap;

    use crate::{
        data::player::Player,
        utils::{
            input::select_from_vector,
            tui::{checkmark, table_from_csv},
        },
    };

    pub fn list() -> BTreeMap<Weapon, usize> {
        BTreeMap::from([
            (Weapon::WoodenSword, 10),
            (Weapon::BronzeSword, 50),
            (Weapon::IronSword, 100),
            (Weapon::SteelSword, 500),
            (Weapon::MysticSword, 1_000),
            (Weapon::WizardStaff, 10_000),
        ])
    }

    pub fn table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owned".to_string()];

        for (flag, price) in list() {
            let owned = player.weapons.get(&flag).owns;

            let string = format!("{},{},{}", flag.name(), price, checkmark(owned));
            strings.push(string)
        }

        table_from_csv(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn picker() -> Weapon {
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

    pub fn buy(player: &mut Player, weapon: Weapon, payment: bool) -> crate::Result<()> {
        let shop = list();
        let price = shop.get(&weapon).expect("Item not found in hashmap.");

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;

            if gold < *price {
                return Err(crate::InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= *price;
        }

        let owns_item = &mut player.weapons.get(&weapon).owns;

        if *owns_item {
            return Err(crate::InventoryError::ItemOwned.boxed());
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, weapon: Weapon, payment: bool) -> crate::Result<()> {
        let shop: BTreeMap<Weapon, usize> = list();
        let price: &usize = shop.get(&weapon).expect("Item not found in hashmap.");
        let owns_item = &mut player.weapons.get(&weapon).owns;

        if !*owns_item {
            return Err(crate::InventoryError::ItemNotOwned.boxed());
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
