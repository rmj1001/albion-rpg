use crate::{
    data::{player::Player, xp::XPType},
    prelude::*,
};

enum Operation {
    Add,
    Subtract,
}

pub fn main(player: &mut Player) {
    page_header("Developer Mode - XP Manager", Instructions::Keyboard);

    player.xp.table();

    let xp_category = select(
        &[
            "1. Combat",
            "2. Fishing",
            "3. Cooking",
            "4. Woodcutting",
            "5. Mining",
            "6. Smithing",
            "7. Thieving",
            "NAV: Go Back",
        ],
        None,
    );

    let mut xp_type: XPType = XPType::Combat;

    match xp_category {
        0 => xp_type = XPType::Combat,
        1 => xp_type = XPType::Fishing,
        2 => xp_type = XPType::Cooking,
        3 => xp_type = XPType::Woodcutting,
        4 => xp_type = XPType::Mining,
        5 => xp_type = XPType::Smithing,
        6 => xp_type = XPType::Thieving,
        7 => super::d1_developer_menu::main(player),
        _ => out_of_bounds(),
    };

    let operation_category: usize = select(&["1. Add", "2. Subtract", "3. Cancel"], None);
    let mut operation: Operation = Operation::Add;

    match operation_category {
        0 => operation = Operation::Add,
        1 => operation = Operation::Subtract,
        2 => main(player),
        _ => out_of_bounds(),
    }

    let amount_result: Result<usize> = input_generic("Amount > ");
    let mut amount: usize = 0;

    match amount_result {
        Ok(number) => amount = number,
        Err(error) => {
            error.failure();
            main(player);
        }
    }

    let result = match operation {
        Operation::Add => player.xp.add(xp_type, amount),
        Operation::Subtract => player.xp.subtract(xp_type, amount),
    };

    match result {
        Ok(_) => success(None),
        Err(error) => error.failure(),
    }

    main(player);
}
