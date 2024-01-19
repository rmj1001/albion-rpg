use crate::{
    lib::{
        input::{out_of_bounds, select_from_str_array},
        math::{generic_calculator, Operation},
        tui::{page_header, press_enter_to_continue, HeaderSubtext},
    },
    user::{profile::UserProfile, xp::XPType},
};

pub fn main(user: &mut UserProfile) {
    page_header("Developer Mode - XP Manager", HeaderSubtext::Keyboard);

    user.xp.print_table();

    let xp_category = select_from_str_array(
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
        7 => super::d1_developer_menu::main(user),
        _ => out_of_bounds(None),
    };

    let calculation = generic_calculator::<usize>();

    if let Operation::Cancel = calculation {
        println!("\nCancelling.");
        press_enter_to_continue();
        main(user);
    }

    // Return early if the operation was cancelled.
    let result: Result<(), &str> = match calculation {
        Operation::Add(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::Subtract(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::Multiply(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::Divide(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::Cancel => Ok(()),
        Operation::Invalid => Err(""),
    };

    match result {
        Ok(_) => {
            println!("\nOperation successful.");
            press_enter_to_continue();
            main(user);
        }
        Err(_) => {
            main(user);
        }
    }
}
