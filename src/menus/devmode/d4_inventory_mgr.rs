use crate::{
    player::inventory::{armor::ArmorItemFlag, items::InventoryItemFlag, weapons::WeaponItemFlag},
    player::profile::Player,
    utils::{
        input::{input_generic, select_from_str_array, select_from_vector},
        messages::*,
        tui::{page_header, HeaderSubtext},
    },
};

pub fn main(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager", HeaderSubtext::None);

    let manager_option = select_from_str_array(&["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"], None);

    match manager_option {
        0 => items_manager(player),
        1 => weapons_manager(player),
        2 => armor_manager(player),
        3 => super::d1_developer_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn items_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Items", HeaderSubtext::None);

    player.inventory.print_table();

    let buysell = select_from_str_array(&["1. Add", "2. Subtract", "NAV: Go Back"], None);

    match buysell {
        0 => add_item(player),
        1 => subtract_item(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn add_item(player: &mut Player) {
        let item_flag = get_item(player);
        let quantity_result = get_quantity();
        let mut quantity: usize = 0;

        match quantity_result {
            Ok(number) => quantity = number,
            Err(_) => {
                invalid_input(None, Some("number"), true);
                items_manager(player);
            }
        }

        let result = player
            .inventory
            .purchase(&mut player.bank.wallet, &item_flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                items_manager(player);
            }
            Err(message) => {
                failure(message);
                items_manager(player);
            }
        }
    }

    pub fn subtract_item(player: &mut Player) {
        let item_flag = get_item(player);
        let quantity_result = get_quantity();
        let mut quantity: usize = 0;

        match quantity_result {
            Ok(number) => quantity = number,
            Err(_) => {
                invalid_input(None, Some("number"), true);
                items_manager(player);
            }
        }

        let result = player
            .inventory
            .sell(&mut player.bank.wallet, &item_flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                items_manager(player);
            }
            Err(message) => {
                failure(message);
                items_manager(player);
            }
        }
    }

    fn get_item(player: &mut Player) -> InventoryItemFlag {
        let item_names: Vec<String> = vec![
            player.inventory.bait.name.to_string(),
            player.inventory.seeds.name.to_string(),
            player.inventory.furs.name.to_string(),
            player.inventory.fish.name.to_string(),
            player.inventory.wood.name.to_string(),
            player.inventory.ore.name.to_string(),
            player.inventory.ingots.name.to_string(),
            player.inventory.potions.name.to_string(),
            player.inventory.rubies.name.to_string(),
            player.inventory.magic_scrolls.name.to_string(),
            player.inventory.bones.name.to_string(),
            player.inventory.dragon_hides.name.to_string(),
            player.inventory.runic_tablets.name.to_string(),
            String::from("NAV: Cancel"),
        ];

        let select = select_from_vector(item_names, None);

        if select == 13 {
            cancelling();
            items_manager(player);
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

    fn get_quantity() -> Result<usize, &'static str> {
        input_generic::<usize>("Quantity:")
    }
}

fn weapons_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Weapons", HeaderSubtext::None);

    player.weapons.print_table();

    let buysell = select_from_str_array(&["1. Own Weapon", "2. Disown Weapon", "NAV: Go Back"], None);

    match buysell {
        0 => own_weapon(player),
        1 => disown_weapon(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn own_weapon(player: &mut Player) {
        let item = get_item(player);

        let result = player.weapons.purchase(&mut player.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(player);
            }
            Err(message) => {
                failure(message);
                weapons_manager(player);
            }
        }
    }

    pub fn disown_weapon(player: &mut Player) {
        let item = get_item(player);

        let result = player.weapons.sell(&mut player.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                weapons_manager(player);
            }
            Err(message) => {
                failure(message);
                weapons_manager(player);
            }
        }
    }

    fn get_item(player: &mut Player) -> WeaponItemFlag {
        let items: Vec<String> = vec![
            player.weapons.wooden_sword.name.to_string(),
            player.weapons.bronze_sword.name.to_string(),
            player.weapons.iron_sword.name.to_string(),
            player.weapons.steel_sword.name.to_string(),
            player.weapons.mystic_sword.name.to_string(),
            player.weapons.wizard_staff.name.to_string(),
            "NAV: Cancel".to_string(),
        ];

        let length = items.len();

        let select = select_from_vector(items, None);

        if select == length - 1 {
            cancelling();
            weapons_manager(player);
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

fn armor_manager(player: &mut Player) {
    page_header("Developer Mode - Inventory Manager - Armor", HeaderSubtext::None);

    player.armor.print_table();

    let buysell = select_from_str_array(&["1. Own Armor", "2. Disown Armor", "NAV: Go Back"], None);

    match buysell {
        0 => own_armor(player),
        1 => disown_armor(player),
        2 => main(player),
        _ => out_of_bounds(),
    }

    pub fn own_armor(player: &mut Player) {
        let item = get_item(player);

        let result = player.armor.purchase(&mut player.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(player);
            }
            Err(message) => {
                failure(message);
                armor_manager(player);
            }
        }
    }

    pub fn disown_armor(player: &mut Player) {
        let item = get_item(player);

        let result = player.armor.sell(&mut player.bank.wallet, item, false);

        match result {
            Ok(_) => {
                success();
                armor_manager(player);
            }
            Err(message) => {
                failure(message);
                armor_manager(player);
            }
        }
    }

    fn get_item(player: &mut Player) -> ArmorItemFlag {
        let items: Vec<String> = vec![
            player.armor.leather.name.to_string(),
            player.armor.bronze.name.to_string(),
            player.armor.iron.name.to_string(),
            player.armor.steel.name.to_string(),
            player.armor.dragonhide.name.to_string(),
            player.armor.mystic.name.to_string(),
            "NAV: Cancel".to_string(),
        ];

        let length = items.len();

        let select = select_from_vector(items, None);

        if select == length - 1 {
            cancelling();
            armor_manager(player);
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
