pub mod shop {
    use crate::data::inventory::items::Item;
    use std::collections::BTreeMap;

    use crate::{
        data::player::Player,
        utils::{
            input::{input_generic, select_from_vector},
            tui::table_from_csv,
        },
        InventoryError,
    };

    type Price = usize;

    pub fn list() -> BTreeMap<Item, Price> {
        BTreeMap::from([
            (Item::Bait, 1),
            (Item::Seeds, 1),
            (Item::Furs, 50),
            (Item::Fish, 5),
            (Item::Food, 10),
            (Item::Wood, 10),
            (Item::Ore, 15),
            (Item::Ingots, 30),
            (Item::Potions, 20),
            (Item::Rubies, 100),
            (Item::MagicScrolls, 200),
            (Item::Bones, 10),
            (Item::DragonHides, 50),
            (Item::RunicTablets, 300),
        ])
    }

    pub fn table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Quantity,,Item,Price,Quantity".to_string()];

        type ShopItem = (Item, usize, usize);
        type Pair = (ShopItem, ShopItem);

        let mut pairs: Vec<Pair> = vec![];
        let mut current_pair: Pair = ((Item::Bait, 0, 0), (Item::Bait, 0, 0));
        let mut index: usize = 0;

        for (flag, price) in list() {
            let quantity = player.items.get(flag);

            if index == 0 {
                current_pair.0 = (flag, price, *quantity);
                index += 1;
            } else if index == 1 {
                current_pair.1 = (flag, price, *quantity);
                pairs.push(current_pair);
                index = 0;
            }
        }

        for (item1, item2) in pairs {
            strings.push(format!(
                "{},{},{},,{},{},{}",
                item1.0.name(),
                item1.1,
                item1.2,
                item2.0.name(),
                item2.1,
                item2.2
            ));
        }

        table_from_csv(strings);
    }

    pub fn build_transaction() -> crate::Result<(Item, usize)> {
        let item = picker();
        let quantity_result = input_generic::<usize>("Quantity:");

        match quantity_result {
            Ok(quantity) => Ok((item, quantity)),
            Err(error) => Err(error),
        }
    }

    pub fn picker() -> Item {
        let shop = list();
        let items = shop.keys();
        let item_names: Vec<String> = items.map(|item| item.name().to_string()).collect();

        let selector = select_from_vector(item_names.clone(), None);
        let selected_item = item_names
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        *list()
            .iter()
            .find(|item| item.0.name() == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
    }

    pub fn buy(player: &mut Player, flag: Item, quantity: usize, use_wallet: bool) -> crate::Result<()> {
        let shop = list();
        let price: Price = *shop.get(&flag).expect("Item not found in hashmap.");

        if use_wallet {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let price = quantity * price;

            if gold < price {
                return Err(InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= price;
        }

        let item = player.items.get(flag);

        *item += quantity;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: Item, quantity: usize, use_wallet: bool) -> crate::Result<()> {
        let shop = list();
        let price: Price = *shop.get(&flag).expect("Item not found in hashmap.");
        let item = player.items.get(flag);

        if *item == 0 || *item < quantity {
            return Err(InventoryError::NotEnoughItem(flag.name().to_string()).boxed());
        }

        *item -= quantity;

        if use_wallet {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = quantity * (price / 2);

            *wallet += price;
        }

        Ok(())
    }
}
