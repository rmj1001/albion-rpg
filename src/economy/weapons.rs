use crate::data::inventory::weapons::Weapon;
use crate::economy::shop::Item;

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

    pub fn list() -> BTreeMap<Weapon, Item> {
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

        for (flag, item) in list() {
            let owned = owns(player, flag);

            let string = format!("{},{},{}", item.name, item.price, checkmark(*owned));
            strings.push(string)
        }

        table_from_csv(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn picker() -> Weapon {
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

    pub fn owns<'a>(player: &'a mut Player, weapon: Weapon) -> &'a mut bool {
        match weapon {
            Weapon::BronzeSword => &mut player.weapons.bronze_sword.owns,
            Weapon::IronSword => &mut player.weapons.iron_sword.owns,
            Weapon::MysticSword => &mut player.weapons.mystic_sword.owns,
            Weapon::SteelSword => &mut player.weapons.steel_sword.owns,
            Weapon::WizardStaff => &mut player.weapons.wizard_staff.owns,
            Weapon::WoodenSword => &mut player.weapons.wooden_sword.owns,
        }
    }

    pub fn buy(player: &mut Player, weapon: Weapon, payment: bool) -> crate::Result<()> {
        let shop = list();
        let item: &Item = shop.get(&weapon).expect("Item not found in hashmap.");

        if payment {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let price = item.price;

            if gold < price {
                return Err(crate::InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= price;
        }

        let owns_item = owns(player, weapon);

        if *owns_item {
            return Err(crate::InventoryError::ItemOwned.boxed());
        }

        *owns_item = true;
        Ok(())
    }

    pub fn sell(player: &mut Player, weapon: Weapon, payment: bool) -> crate::Result<()> {
        let shop: BTreeMap<Weapon, Item> = list();
        let shop_item: &Item = shop.get(&weapon).expect("Item not found in hashmap.");
        let owns_item: &mut bool = owns(player, weapon);

        if !*owns_item {
            return Err(crate::InventoryError::ItemNotOwned.boxed());
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
