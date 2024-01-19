use crate::{
    lib::{
        input::{
            input_generic, invalid_input, out_of_bounds, select_from_str_array, select_from_vector,
        },
        tui::{page_header, press_enter_to_continue},
    },
    user::{inventory::InventoryItemFlag, profile::UserProfile},
};

pub fn main(user: &mut UserProfile) {
    page_header("Trading Post", crate::lib::tui::HeaderSubtext::None);

    user.inventory.print_table();
    println!("Gold: {}\n", user.bank.wallet);

    let buysell = select_from_str_array(&["1. Purchase", "2. Sell", "NAV: Go Back"], None);

    match buysell {
        0 => purchase(user),
        1 => sell(user),
        2 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(None),
    }

    crate::menus::game_menu::main(user);
}

pub fn purchase(user: &mut UserProfile) {
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
        .purchase(&mut user.bank.wallet, &item_flag, quantity);

    match result {
        Ok(_) => {
            println!("Operation successful.");
            press_enter_to_continue();
            main(user);
        }
        Err(message) => {
            eprintln!("{}", message);
            press_enter_to_continue();
            main(user);
        }
    }
}

pub fn sell(user: &mut UserProfile) {
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
        .sell(&mut user.bank.wallet, &item_flag, quantity);

    match result {
        Ok(_) => {
            println!("\nOperation successful.");
            press_enter_to_continue();
            main(user);
        }
        Err(message) => {
            eprintln!("\n{}", message);
            press_enter_to_continue();
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
        println!("\nCancelling.");
        press_enter_to_continue();
        main(user);
        return InventoryItemFlag::Invalid;
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
        _ => InventoryItemFlag::Invalid,
    }
}

fn get_quantity<'a>() -> Result<usize, &'a str> {
    input_generic::<usize>("Quantity:")
}
