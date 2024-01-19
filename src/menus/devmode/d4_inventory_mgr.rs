use crate::{
    lib::{
        input::{input_generic, select_from_str_array, select_from_vector},
        messages::*,
        tui::{page_header, HeaderSubtext},
    },
    user::{
        armor::ArmorItemFlag, inventory::InventoryItemFlag, profile::UserProfile,
        weapons::WeaponItemFlag,
    },
};

pub fn main(user: &mut UserProfile) {
    page_header("Developer Mode - Inventory Manager", HeaderSubtext::None);

    let manager_option = select_from_str_array(
        &["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"],
        None,
    );

    match manager_option {
        0 => items_manager(user),
        1 => weapons_manager(user),
        2 => armor_manager(user),
        3 => super::d1_developer_menu::main(user),
        _ => out_of_bounds(None),
    }
}

fn items_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Items",
        HeaderSubtext::None,
    );

    user.inventory.print_table();

    let buysell = select_from_str_array(&["1. Add", "2. Subtract", "NAV: Go Back"], None);

    match buysell {
        0 => add_item(user),
        1 => subtract_item(user),
        2 => main(user),
        _ => out_of_bounds(None),
    }

    pub fn add_item(user: &mut UserProfile) {
        let item_flag = get_item(user);
        let quantity_result = get_quantity();
        let mut quantity: usize = 0;

        match quantity_result {
            Ok(number) => quantity = number,
            Err(_) => {
                invalid_input(None, Some("number"), true);
                items_manager(user);
            }
        }

        let result = user
            .inventory
            .purchase(&mut user.bank.wallet, &item_flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                items_manager(user);
            }
            Err(message) => {
                failure(message);
                items_manager(user);
            }
        }
    }

    pub fn subtract_item(user: &mut UserProfile) {
        let item_flag = get_item(user);
        let quantity_result = get_quantity();
        let mut quantity: usize = 0;

        match quantity_result {
            Ok(number) => quantity = number,
            Err(_) => {
                invalid_input(None, Some("number"), true);
                items_manager(user);
            }
        }

        let result = user
            .inventory
            .sell(&mut user.bank.wallet, &item_flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                items_manager(user);
            }
            Err(message) => {
                failure(message);
                items_manager(user);
            }
        }
    }

    fn get_item(user: &mut UserProfile) -> InventoryItemFlag {
        let item_names: Vec<String> = vec![
            user.inventory.bait.name.to_string(),
            user.inventory.seeds.name.to_string(),
            user.inventory.furs.name.to_string(),
            user.inventory.fish.name.to_string(),
            user.inventory.wood.name.to_string(),
            user.inventory.ore.name.to_string(),
            user.inventory.ingots.name.to_string(),
            user.inventory.potions.name.to_string(),
            user.inventory.rubies.name.to_string(),
            user.inventory.magic_scrolls.name.to_string(),
            user.inventory.bones.name.to_string(),
            user.inventory.dragon_hides.name.to_string(),
            user.inventory.runic_tablets.name.to_string(),
            String::from("NAV: Cancel"),
        ];

        let select = select_from_vector(item_names, None);

        if select == 13 {
            cancelling();
            items_manager(user);
            return InventoryItemFlag::InvalidItem;
        }

        match select {
            0 => InventoryItemFlag::Bait,
            1 => InventoryItemFlag::Seeds,
            2 => InventoryItemFlag::Furs,
            3 => InventoryItemFlag::Fish,
            4 => InventoryItemFlag::Wood,
            5 => InventoryItemFlag::Ore,
            6 => InventoryItemFlag::Ingots,
            7 => InventoryItemFlag::Potions,
            8 => InventoryItemFlag::Rubies,
            9 => InventoryItemFlag::MagicScrolls,
            10 => InventoryItemFlag::Bones,
            11 => InventoryItemFlag::DragonHides,
            12 => InventoryItemFlag::RunicTablets,
            _ => InventoryItemFlag::InvalidItem,
        }
    }

    fn get_quantity<'a>() -> Result<usize, &'a str> {
        input_generic::<usize>("Quantity:")
    }
}

fn weapons_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Weapons",
        HeaderSubtext::None,
    );

    user.weapons.print_table();

    let buysell =
        select_from_str_array(&["1. Own Weapon", "2. Disown Weapon", "NAV: Go Back"], None);

    match buysell {
        0 => own_weapon(user),
        1 => disown_weapon(user),
        2 => main(user),
        _ => out_of_bounds(None),
    }

    pub fn own_weapon(user: &mut UserProfile) {
        let item = get_item(user);

        let result = user.weapons.purchase(&mut user.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(user);
            }
            Err(message) => {
                failure(message);
                weapons_manager(user);
            }
        }
    }

    pub fn disown_weapon(user: &mut UserProfile) {
        let item = get_item(user);

        let result = user.weapons.sell(&mut user.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(user);
            }
            Err(message) => {
                failure(message);
                weapons_manager(user);
            }
        }
    }

    fn get_item(user: &mut UserProfile) -> WeaponItemFlag {
        let items: Vec<String> = vec![
            user.weapons.wooden_sword.name.to_string(),
            user.weapons.bronze_sword.name.to_string(),
            user.weapons.iron_sword.name.to_string(),
            user.weapons.steel_sword.name.to_string(),
            user.weapons.mystic_sword.name.to_string(),
            user.weapons.wizard_staff.name.to_string(),
            "NAV: Cancel".to_string(),
        ];

        let length = items.len();

        let select = select_from_vector(items, None);

        if select == length - 1 {
            cancelling();
            weapons_manager(user);
            return WeaponItemFlag::InvalidItem;
        }

        match select {
            0 => WeaponItemFlag::WoodenSword,
            1 => WeaponItemFlag::BronzeSword,
            2 => WeaponItemFlag::IronSword,
            3 => WeaponItemFlag::SteelSword,
            4 => WeaponItemFlag::MysticSword,
            5 => WeaponItemFlag::WizardStaff,
            _ => WeaponItemFlag::InvalidItem,
        }
    }
}

fn armor_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Armor",
        HeaderSubtext::None,
    );

    user.armor.print_table();

    let buysell = select_from_str_array(&["1. Own Armor", "2. Disown Armor", "NAV: Go Back"], None);

    match buysell {
        0 => own_armor(user),
        1 => disown_armor(user),
        2 => main(user),
        _ => out_of_bounds(None),
    }

    pub fn own_armor(user: &mut UserProfile) {
        let item = get_item(user);

        let result = user.armor.purchase(&mut user.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(user);
            }
            Err(message) => {
                failure(message);
                armor_manager(user);
            }
        }
    }

    pub fn disown_armor(user: &mut UserProfile) {
        let item = get_item(user);

        let result = user.armor.sell(&mut user.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(user);
            }
            Err(message) => {
                failure(message);
                armor_manager(user);
            }
        }
    }

    fn get_item(user: &mut UserProfile) -> ArmorItemFlag {
        let items: Vec<String> = vec![
            user.armor.leather.name.to_string(),
            user.armor.bronze.name.to_string(),
            user.armor.iron.name.to_string(),
            user.armor.steel.name.to_string(),
            user.armor.dragonhide.name.to_string(),
            user.armor.mystic.name.to_string(),
            "NAV: Cancel".to_string(),
        ];

        let length = items.len();

        let select = select_from_vector(items, None);

        if select == length - 1 {
            cancelling();
            armor_manager(user);
            return ArmorItemFlag::InvalidItem;
        }

        match select {
            0 => ArmorItemFlag::Leather,
            1 => ArmorItemFlag::Bronze,
            2 => ArmorItemFlag::Iron,
            3 => ArmorItemFlag::Steel,
            4 => ArmorItemFlag::DragonHide,
            5 => ArmorItemFlag::Mystic,
            _ => ArmorItemFlag::InvalidItem,
        }
    }
}
