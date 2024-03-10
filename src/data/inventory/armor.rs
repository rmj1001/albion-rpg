use crate::{data::player::Player, prelude::*, InventoryError};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Armor {
    Leather,
    Bronze,
    Iron,
    Steel,
    Dragonhide,
    Mystic,
}

impl Armor {
    pub fn name(&self) -> &'static str {
        match self {
            Armor::Leather => "Leather Armor",
            Armor::Bronze => "Bronze Armor",
            Armor::Iron => "Iron Armor",
            Armor::Steel => "Steel Armor",
            Armor::Dragonhide => "Dragonhide Armor",
            Armor::Mystic => "Mystic Armor",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArmorData {
    pub owns: bool,
    pub defense: usize,
    pub durability: usize,
    pub default_durability: usize,
    pub equipped: bool,
    pub flag: Armor,
}

impl Default for ArmorData {
    fn default() -> Self {
        Self {
            owns: false,
            equipped: false,
            defense: 5,
            durability: 50,
            default_durability: 50,
            flag: Armor::Leather,
        }
    }
}

impl ArmorData {
    pub fn new(defense: usize, durability: usize, flag: Armor) -> Self {
        Self {
            defense,
            durability,
            flag,
            ..Default::default()
        }
    }

    pub fn decrease_durability(&mut self) {
        let random_damage = thread_rng().gen_range(1..5);

        if self.durability < random_damage {
            self.break_armor();
        } else {
            self.durability -= random_damage;
        }
    }

    pub fn break_armor(&mut self) {
        println!("Your {} broke!", self.flag.name());
        self.owns = false;
        self.durability = self.default_durability;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArmorInventory {
    pub leather: ArmorData,
    pub bronze: ArmorData,
    pub iron: ArmorData,
    pub steel: ArmorData,
    pub dragonhide: ArmorData,
    pub mystic: ArmorData,
}

impl ArmorInventory {
    pub fn new() -> ArmorInventory {
        ArmorInventory {
            leather: ArmorData::new(10, 100, Armor::Leather),
            bronze: ArmorData::new(30, 200, Armor::Bronze),
            iron: ArmorData::new(50, 300, Armor::Iron),
            steel: ArmorData::new(100, 500, Armor::Steel),
            dragonhide: ArmorData::new(200, 500, Armor::Dragonhide),
            mystic: ArmorData::new(1_000, 10_000, Armor::Mystic),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn table(&self) {
        fn entry(armor: &ArmorData) -> String {
            format!(
                "{},{},{},{},{}",
                armor.flag.name(),
                checkmark(armor.owns),
                checkmark(armor.equipped),
                armor.defense,
                armor.durability,
            )
        }

        table_from_csv(vec![
            "Armor,Owned,Equipped,Defense,Durability".to_string(),
            entry(&self.leather),
            entry(&self.bronze),
            entry(&self.iron),
            entry(&self.steel),
            entry(&self.dragonhide),
            entry(&self.mystic),
        ])
    }

    pub fn get(&mut self, item_flag: &Armor) -> &mut ArmorData {
        match item_flag {
            Armor::Bronze => &mut self.bronze,
            Armor::Dragonhide => &mut self.dragonhide,
            Armor::Iron => &mut self.iron,
            Armor::Leather => &mut self.leather,
            Armor::Mystic => &mut self.mystic,
            Armor::Steel => &mut self.steel,
        }
    }

    /// For use in developer mode only
    pub fn toggle_own(&mut self, item_flag: &Armor) {
        let item = self.get(item_flag);
        item.owns = !item.owns;
    }
}

// -------------------------------------------------- Economy -------------------------------------------------- //

impl ArmorInventory {
    fn shop() -> BTreeMap<Armor, usize> {
        BTreeMap::from([
            (Armor::Leather, 100),
            (Armor::Bronze, 300),
            (Armor::Iron, 1_000),
            (Armor::Steel, 5_000),
            (Armor::Dragonhide, 10_000),
            (Armor::Mystic, 20_000),
        ])
    }

    pub fn shop_table(player: &mut Player) {
        let mut strings: Vec<String> = vec!["Item,Price,Owns".to_string()];

        Self::shop().iter().for_each(|(flag, price)| {
            let string = format!("{},{},{}", flag.name(), price, checkmark(player.armor.get(flag).owns));
            strings.push(string)
        });

        table_from_csv(strings);
        println!("Gold: {}\n", player.bank.wallet);
    }

    pub fn select() -> Armor {
        let shop = Self::shop();
        let items: Vec<String> = shop.keys().map(|flag| flag.name().to_string()).collect();

        let selector = select_from_vector(items.clone(), None);
        let selected_item = items
            .get(selector)
            .expect("This shouldn't select a vector item out of bounds.")
            .to_string();

        Self::shop()
            .iter()
            .find(|item| item.0.name() == selected_item)
            .map(|item| item.0)
            .expect("Should return an Item Flag")
            .clone()
    }

    pub fn buy(player: &mut Player, flag: &Armor, payment: bool) -> crate::Result<()> {
        let shop = Self::shop();
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
        let shop: BTreeMap<Armor, usize> = Self::shop();
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
