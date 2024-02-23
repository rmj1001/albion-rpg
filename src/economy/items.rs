use crate::economy::shop::Item;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Items {
    Bait,
    Seeds,
    Furs,
    Fish,
    Food,
    Wood,
    Ore,
    Ingots,
    Potions,
    Rubies,
    MagicScrolls,
    Bones,
    DragonHides,
    RunicTablets,
    InvalidItem,
}

pub mod shop {
    use super::{Item, Items};
    use std::collections::BTreeMap;

    use crate::{
        data::{inventory::items::MundaneInventory, player::Player},
        utils::{
            input::{input_generic, select_from_vector},
            tui::table_from_csv,
        },
        InventoryError,
    };

    pub fn shop_list() -> BTreeMap<Items, Item> {
        BTreeMap::from([
            (Items::Bait, Item::new("Bait", 1)),
            (Items::Seeds, Item::new("Seeds", 1)),
            (Items::Furs, Item::new("Fur", 50)),
            (Items::Fish, Item::new("Fish", 5)),
            (Items::Food, Item::new("Food", 10)),
            (Items::Wood, Item::new("Wood", 10)),
            (Items::Ore, Item::new("Ore", 15)),
            (Items::Ingots, Item::new("Ingot", 30)),
            (Items::Potions, Item::new("Potion", 20)),
            (Items::Rubies, Item::new("Ruby", 100)),
            (Items::MagicScrolls, Item::new("Magic Scroll", 200)),
            (Items::Bones, Item::new("Bone", 10)),
            (Items::DragonHides, Item::new("Dragon Hide", 50)),
            (Items::RunicTablets, Item::new("Runic Tablet", 300)),
        ])
    }

    pub fn table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Quantity".to_string()];

        for (flag, item) in shop_list() {
            let quantity = shop_quantity(player, flag);

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

    pub fn build_transaction() -> (Items, usize) {
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

        let quantity_result = input_generic::<usize>("Quantity:");

        match quantity_result {
            Ok(quantity) => (item, quantity),
            Err(error) => panic!("{}", error),
        }
    }

    pub fn get_item() -> Items {
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

    pub fn shop_quantity(player: &mut Player, flag: Items) -> Option<&mut usize> {
        let items: &mut MundaneInventory = &mut player.items;

        let item: Option<&mut usize> = match flag {
            Items::Bait => Some(&mut items.bait),
            Items::Bones => Some(&mut items.bones),
            Items::DragonHides => Some(&mut items.dragon_hides),
            Items::Fish => Some(&mut items.fish),
            Items::Food => Some(&mut items.food),
            Items::Furs => Some(&mut items.furs),
            Items::Ingots => Some(&mut items.ingots),
            Items::InvalidItem => None,
            Items::MagicScrolls => Some(&mut items.magic_scrolls),
            Items::Ore => Some(&mut items.ore),
            Items::Potions => Some(&mut items.potions),
            Items::Rubies => Some(&mut items.rubies),
            Items::RunicTablets => Some(&mut items.runic_tablets),
            Items::Seeds => Some(&mut items.seeds),
            Items::Wood => Some(&mut items.wood),
        };

        item
    }

    pub fn purchase(player: &mut Player, flag: Items, quantity: usize, use_wallet: bool) -> crate::Result<()> {
        let shop = shop_list();
        let item: &Item = shop.get(&flag).expect("Item not found in hashmap.");

        if use_wallet {
            let gold: usize = player.bank.wallet;
            let wallet: &mut usize = &mut player.bank.wallet;
            let price = quantity * item.price;

            if gold < price {
                return Err(InventoryError::NotEnoughGold.boxed());
            }

            *wallet -= price;
        }

        let player_quantity =
            shop_quantity(player, flag).expect("Don't use the InvalidItem variant in the shop hash map.");

        *player_quantity += quantity;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: Items, quantity: usize, use_wallet: bool) -> crate::Result<()> {
        let shop: BTreeMap<Items, Item> = shop_list();
        let shop_item: &Item = shop.get(&flag).expect("Item not found in hashmap.");
        let item: &mut usize =
            shop_quantity(player, flag).expect("Don't use the InvalidItem variant in the shop hash map.");

        if *item == 0 || *item < quantity {
            return Err(InventoryError::NotEnoughItem(shop_item.name.clone()).boxed());
        }

        *item -= quantity;

        if use_wallet {
            let wallet: &mut usize = &mut player.bank.wallet;
            let price: usize = quantity * (shop_item.price / 2);

            *wallet += price;
        }

        Ok(())
    }
}
