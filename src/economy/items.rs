use std::hash::Hash;

#[derive(Hash)]
pub struct Item {
    pub name: String,
    pub price: usize,
}

impl Item {
    pub fn new(name: &str, price: usize) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }
}

pub enum GuildItemNames {
    Fish,
    CookedFish,
    Wood,
    Ore,
    Ingots,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ItemFlag {
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
    use super::{Item, ItemFlag};
    use std::collections::BTreeMap;

    use crate::{
        data::{inventory::items::MundaneInventory, player::Player},
        utils::{
            input::{input_generic, select_from_vector},
            tui::table_from_csv,
        },
    };

    pub fn shop_list() -> BTreeMap<ItemFlag, Item> {
        BTreeMap::from([
            (ItemFlag::Bait, Item::new("Bait", 1)),
            (ItemFlag::Seeds, Item::new("Seeds", 1)),
            (ItemFlag::Furs, Item::new("Fur", 50)),
            (ItemFlag::Fish, Item::new("Fish", 5)),
            (ItemFlag::Food, Item::new("Food", 10)),
            (ItemFlag::Wood, Item::new("Wood", 10)),
            (ItemFlag::Ore, Item::new("Ore", 15)),
            (ItemFlag::Ingots, Item::new("Ingot", 30)),
            (ItemFlag::Potions, Item::new("Potion", 20)),
            (ItemFlag::Rubies, Item::new("Ruby", 100)),
            (ItemFlag::MagicScrolls, Item::new("Magic Scroll", 200)),
            (ItemFlag::Bones, Item::new("Bone", 10)),
            (ItemFlag::DragonHides, Item::new("Dragon Hide", 50)),
            (ItemFlag::RunicTablets, Item::new("Runic Tablet", 300)),
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

    pub fn build_transaction() -> (ItemFlag, usize) {
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

    pub fn get_item() -> ItemFlag {
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

    pub fn shop_quantity(player: &mut Player, flag: ItemFlag) -> Option<&mut usize> {
        let items: &mut MundaneInventory = &mut player.items;

        let item: Option<&mut usize> = match flag {
            ItemFlag::Bait => Some(&mut items.bait),
            ItemFlag::Bones => Some(&mut items.bones),
            ItemFlag::DragonHides => Some(&mut items.dragon_hides),
            ItemFlag::Fish => Some(&mut items.fish),
            ItemFlag::Food => Some(&mut items.food),
            ItemFlag::Furs => Some(&mut items.furs),
            ItemFlag::Ingots => Some(&mut items.ingots),
            ItemFlag::InvalidItem => None,
            ItemFlag::MagicScrolls => Some(&mut items.magic_scrolls),
            ItemFlag::Ore => Some(&mut items.ore),
            ItemFlag::Potions => Some(&mut items.potions),
            ItemFlag::Rubies => Some(&mut items.rubies),
            ItemFlag::RunicTablets => Some(&mut items.runic_tablets),
            ItemFlag::Seeds => Some(&mut items.seeds),
            ItemFlag::Wood => Some(&mut items.wood),
        };

        item
    }

    pub fn purchase(player: &mut Player, flag: ItemFlag, quantity: usize, use_wallet: bool) -> Result<(), &str> {
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

        let player_quantity =
            shop_quantity(player, flag).expect("Don't use the InvalidItem variant in the shop hash map.");

        *player_quantity += quantity;
        Ok(())
    }

    pub fn sell(player: &mut Player, flag: ItemFlag, quantity: usize, use_wallet: bool) -> Result<(), String> {
        let shop: BTreeMap<ItemFlag, Item> = shop_list();
        let shop_item: &Item = shop.get(&flag).expect("Item not found in hashmap.");
        let item: &mut usize =
            shop_quantity(player, flag).expect("Don't use the InvalidItem variant in the shop hash map.");

        if *item == 0 || *item < quantity {
            let error = format!("Not enough {}.", shop_item.name);
            return Err(error);
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
