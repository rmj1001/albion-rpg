use crate::{
    lib::{
        input::{self, out_of_bounds, prompt_input, select_from_vector, selector, yes_or_no},
        tui::{self, page_header, press_enter_to_continue, HeaderInstructions},
    },
    user::xp::{MathOp, XPType},
};

use crate::user::{
    bank::{Bank, BankAccount, BankResult},
    profile::{ProfileRetrievalResult, UserProfile},
};

pub fn main(user: &mut UserProfile) {
    page_header("Developer Mode", HeaderInstructions::Keyboard);

    let choice = selector(
        &[
            "1. Throw a panic",
            "2. Manipulate Inventory",
            "3. Manipulate XP",
            "4. Manipulate Banks",
            "5. Manage User Profiles",
            "6. Disable developer mode",
            "NAV: Go Back",
        ],
        0,
        None,
    );

    match choice {
        0 => panic!("This is a panic!"),
        1 => inventory_manager(user),
        2 => xp_manager(user),
        3 => bank_manager(user),
        4 => user_manager(user),
        5 => disable_developer_mode(user),
        6 => crate::menus::game::main::main(user),
        _ => out_of_bounds(None),
    }
}

pub fn disable_developer_mode(user: &mut UserProfile) {
    page_header("Developer Mode", HeaderInstructions::None);

    let disable_dev_mode = yes_or_no("Are you sure you want to disable developer mode?");

    if !disable_dev_mode {
        println!("\nAborting.");
        press_enter_to_continue();
        main(user);
    }

    user.settings.set_developer(None, false);
    println!("\nDeveloper mode disabled.");
    tui::press_enter_to_continue();

    crate::menus::game::main::main(user);
}

fn user_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - User Manager",
        HeaderInstructions::Keyboard,
    );

    // Listing profiles for printing or deletion
    let profiles: Vec<String> = UserProfile::list_all();

    let choice1 = selector(
        &[
            "1. List Users",
            "2. Delete User",
            "3. View User File",
            "NAV: Go Back",
        ],
        0,
        None,
    );

    match choice1 {
        // listing profiles
        0 => {
            page_header("Developer Mode - User Manager", HeaderInstructions::None);

            for profile_string in &profiles {
                println!("- {}", profile_string);
            }

            println!();
            tui::press_enter_to_continue();

            user_manager(user);
        }

        // deleting profiles
        1 => {
            page_header(
                "Developer Mode - User Manager",
                HeaderInstructions::Keyboard,
            );

            let choice =
                select_from_vector(profiles.clone(), 0, Some("Select a profile to delete"));

            let profile_choice = profiles.get(choice);

            match profile_choice {
                Some(profile_string) => {
                    let delete_profile = yes_or_no(&format!(
                        "Are you sure you want to delete profile '{}'?",
                        profile_string
                    ));

                    if !delete_profile {
                        println!("\nAborting.");
                        press_enter_to_continue();
                        user_manager(user);
                    }

                    if *profile_string == user.settings.username {
                        UserProfile::delete_from_username(&user.settings.username);

                        page_header("Developer Mode - User Manager", HeaderInstructions::None);
                        println!("\nCurrent profile successfully deleted. Logging out.");
                        tui::press_enter_to_continue();

                        crate::menus::accounts::main::main();
                    }

                    UserProfile::delete_from_username(profile_string);

                    page_header("Developer Mode - User Manager", HeaderInstructions::None);
                    println!("\nProfile '{}' successfully deleted.", profile_string);
                    tui::press_enter_to_continue();

                    user_manager(user);
                }

                None => out_of_bounds(None),
            }
        }

        2 => view_user(user),

        3 => main(user),

        _ => out_of_bounds(None),
    }
}

fn view_user(user: &mut UserProfile) {
    page_header(
        "Developer Mode - User Manager - Data Viewer",
        HeaderInstructions::None,
    );
    let choice = select_from_vector(UserProfile::list_all(), 0, Some("Select a user to view"));

    let profiles = UserProfile::list_all();
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let profile_result = UserProfile::retrieve(profile_string);

            match profile_result {
                ProfileRetrievalResult::Some(profile) => {
                    let json_string = profile.to_pretty_json();

                    page_header(
                        &format!("User Profile - {}", profile.settings.username),
                        HeaderInstructions::None,
                    );

                    println!("{}\n", json_string);

                    press_enter_to_continue();
                    user_manager(user);
                }
                ProfileRetrievalResult::None(message) => {
                    println!("\n{}", message);
                    press_enter_to_continue();

                    user_manager(user);
                }
            }
        }
        None => out_of_bounds(None),
    }

    user_manager(user);
}

fn bank_manager(user: &mut UserProfile) {
    let mut account: BankAccount = BankAccount::Account1;

    page_header(
        "Developer Mode - Bank Managert",
        HeaderInstructions::Keyboard,
    );

    println!("Coin Purse: {} Gold", user.gold);
    println!();
    println!("Account 1: {} Gold", user.bank.account1);
    println!("Account 2: {} Gold", user.bank.account2);
    println!("Account 3: {} Gold", user.bank.account3);
    println!("Account 4: {} Gold\n", user.bank.account4);

    let account_choice = selector(
        &[
            "Coin Purse",
            "Account 1",
            "Account 2",
            "Account 3",
            "Account 4",
            "NAV: Go Back",
        ],
        0,
        None,
    );

    match account_choice {
        0 => account = BankAccount::CoinPurse,
        1 => account = BankAccount::Account1,
        2 => account = BankAccount::Account2,
        3 => account = BankAccount::Account3,
        4 => account = BankAccount::Account4,
        5 => main(user),
        _ => out_of_bounds(None),
    }

    let option = selector(&["Add Money", "Subtract Money", "NAV: Cancel"], 0, None);

    if option == 2 {
        main(user);
    }

    let amount_result = input::prompt_input("Amount").parse::<u32>();
    let mut amount: u32 = 0;

    match amount_result {
        Ok(number) => amount = number,
        Err(_) => {
            input::invalid_input(None, None, true);
            bank_manager(user);
        }
    }

    let mut bank_result: BankResult = BankResult::Error("Uninitialized");

    match option {
        // Deposit
        0 => bank_result = Bank::deposit(user, account, amount, true),
        // Withdrawal
        1 => bank_result = Bank::withdraw(user, account, amount, true),
        2 => bank_manager(user),
        _ => out_of_bounds(None),
    }

    match bank_result {
        BankResult::Ok => {
            println!("\nOperation successful.");
            press_enter_to_continue();
            bank_manager(user);
        }

        BankResult::Error(message) => {
            println!("\n{}", message);
            press_enter_to_continue();
            bank_manager(user);
        }
    }
}

fn xp_manager(user: &mut UserProfile) {
    page_header("Developer Mode - XP Manager", HeaderInstructions::Keyboard);

    // TODO: XP Manager
    let xp_category = selector(
        &[
            "Combat",
            "Fishing",
            "Cooking",
            "Woodcutting",
            "Mining",
            "Smithing",
            "Thieving",
            "NAV: Go Back",
        ],
        0,
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
        7 => main(user),
        _ => out_of_bounds(None),
    };

    let calculation = prompt_input("Enter operation. Ex. +1, -1, *1, /1.");
    let chars: Vec<char> = calculation.chars().collect();
    let operator = chars[0];

    match operator {
        '+' => {}
        '-' => {}
        '*' => {}
        '/' => {}
        invalid => {
            input::invalid_input(Some(&invalid.to_string()), None, true);
            xp_manager(user);
        }
    }

    let number_string = format!("{:?}", &chars[1..]);
    let number_result = number_string.trim().parse::<usize>();

    if number_result.is_err() {
        println!("{} is not a valid number", number_string);
        press_enter_to_continue();
        xp_manager(user)
    }

    let mut operation: MathOp = MathOp::Add;

    match operator {
        '+' => operation = MathOp::Add,
        '-' => operation = MathOp::Subtract,
        '*' => operation = MathOp::Multiply,
        '/' => operation = MathOp::Divide,
        _ => {}
    };

    let number = number_result.unwrap();

    let _ = user.xp.arithmetic(xp_type, operation, number as u32);

    main(user);
}

fn inventory_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager",
        HeaderInstructions::None,
    );

    let manager_option = selector(
        &["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"],
        0,
        None,
    );

    match manager_option {
        0 => items_manager(user),
        1 => weapons_manager(user),
        2 => armor_manager(user),
        3 => main(user),
        _ => out_of_bounds(None),
    }

    main(user);
}

fn items_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Items",
        HeaderInstructions::None,
    );

    // TODO: Items Manager

    inventory_manager(user);
}

fn weapons_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Weapons",
        HeaderInstructions::None,
    );

    // TODO: Weapons Manager

    inventory_manager(user);
}

fn armor_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Armor",
        HeaderInstructions::None,
    );

    // TODO: Armor Manager

    inventory_manager(user);
}
