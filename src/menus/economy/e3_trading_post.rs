use crate::{
    player::{inventory::items::InventoryItemFlag, profile::Player},
    utils::{
        input::{input_generic, select_from_str_array, select_from_vector},
        messages::*,
        tui::page_header,
    },
};

pub fn main(player: &mut Player) {
    page_header("Trading Post", crate::utils::tui::HeaderSubtext::None);

    player.inventory.table();
    println!("Gold: {}\n", player.bank.wallet);

    let buysell = select_from_str_array(&["1. Purchase", "2. Sell", "NAV: Go Back"], None);

    match buysell {
        0 => purchase(player),
        1 => sell(player),
        2 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }

    crate::menus::game_menu::main(player);
}

pub fn purchase(player: &mut Player) {
    let item_flag = get_item(player);
    let quantity_result = get_quantity();
    let mut quantity: usize = 0;

    match quantity_result {
        Ok(number) => quantity = number,
        Err(_) => {
            invalid_input(None, Some("number"), true);
            main(player);
        }
    }

    let result = player
        .inventory
        .purchase(&mut player.bank.wallet, &item_flag, quantity, true);

    match result {
        Ok(_) => {
            success();
            main(player);
        }
        Err(message) => {
            failure(message);
            main(player);
        }
    }
}

pub fn sell(player: &mut Player) {
    let item_flag = get_item(player);
    let quantity_result = get_quantity();
    let mut quantity: usize = 0;

    match quantity_result {
        Ok(number) => quantity = number,
        Err(_) => {
            invalid_input(None, Some("number"), true);
            main(player);
        }
    }

    let result = player
        .inventory
        .sell(&mut player.bank.wallet, &item_flag, quantity, true);

    match result {
        Ok(_) => {
            success();
            main(player);
        }
        Err(message) => {
            failure(message);
            main(player);
        }
    }
}

fn get_item(player: &mut Player) -> InventoryItemFlag {
    let item_names: Vec<String> = vec![
        player.inventory.bait.name.to_string(),
        player.inventory.seeds.name.to_string(),
        player.inventory.furs.name.to_string(),
        player.inventory.fish.name.to_string(),
        player.inventory.food.name.to_string(),
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
        main(player);
        return InventoryItemFlag::InvalidItem;
    }

    match select {
        0 => InventoryItemFlag::Bait,
        1 => InventoryItemFlag::Seeds,
        2 => InventoryItemFlag::Furs,
        3 => InventoryItemFlag::Fish,
        4 => InventoryItemFlag::Food,
        5 => InventoryItemFlag::Wood,
        6 => InventoryItemFlag::Ore,
        7 => InventoryItemFlag::Ingots,
        8 => InventoryItemFlag::Potions,
        9 => InventoryItemFlag::Rubies,
        10 => InventoryItemFlag::MagicScrolls,
        11 => InventoryItemFlag::Bones,
        12 => InventoryItemFlag::DragonHides,
        13 => InventoryItemFlag::RunicTablets,
        _ => InventoryItemFlag::InvalidItem,
    }
}

fn get_quantity() -> Result<usize, &'static str> {
    input_generic::<usize>("Quantity:")
}
