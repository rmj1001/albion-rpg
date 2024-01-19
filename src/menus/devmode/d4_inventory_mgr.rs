use crate::{
    lib::{
        input::{input_generic, select_from_str_array, select_from_vector},
        stdmsgs::*,
        tui::{page_header, HeaderSubtext},
    },
    user::{inventory::InventoryItemFlag, profile::UserProfile},
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
                main(user);
            }
        }

        let result = user
            .inventory
            .purchase(&mut user.bank.wallet, &item_flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                add_item(user);
            }
            Err(message) => {
                error(message);
                add_item(user);
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
                main(user);
            }
        }

        let result = user
            .inventory
            .sell(&mut user.bank.wallet, &item_flag, quantity, false);

        match result {
            Ok(_) => {
                success();
                main(user);
            }
            Err(message) => {
                error(message);
                main(user);
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
            main(user);
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

    // TODO: Weapons Manager

    main(user);
}

fn armor_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Armor",
        HeaderSubtext::None,
    );

    // TODO: Armor Manager

    main(user);
}
