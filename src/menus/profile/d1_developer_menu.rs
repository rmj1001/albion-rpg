use crate::{
    lib::{
        input::{out_of_bounds, select_from_vector, selector, yes_or_no},
        math::{generic_calculator, Operation},
        tui::{self, page_header, press_enter_to_continue, HeaderSubtext},
    },
    user::xp::XPType,
};

use crate::user::{
    bank::BankAccount,
    profile::{ProfileRetrievalResult, UserProfile},
};

pub fn main(user: &mut UserProfile) {
    page_header("Developer Mode", HeaderSubtext::Keyboard);

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
        None,
    );

    match choice {
        0 => panic!("This is a panic!"),
        1 => inventory_manager(user),
        2 => xp_manager(user),
        3 => bank_manager(user),
        4 => user_manager(user),
        5 => disable_developer_mode(user),
        6 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(None),
    }
}

pub fn disable_developer_mode(user: &mut UserProfile) {
    page_header("Developer Mode", HeaderSubtext::None);

    let disable_dev_mode = yes_or_no("Are you sure you want to disable developer mode?");

    if !disable_dev_mode {
        println!("\nAborting.");
        press_enter_to_continue();
        main(user);
    }

    user.settings.set_developer(None, false);
    println!("\nDeveloper mode disabled.");
    tui::press_enter_to_continue();

    crate::menus::game_menu::main(user);
}

fn user_manager(user: &mut UserProfile) {
    page_header("Developer Mode - User Manager", HeaderSubtext::Keyboard);

    // Listing profiles for printing or deletion
    let profiles: Vec<String> = UserProfile::list_all();

    let choice1 = selector(
        &[
            "1. List Users",
            "2. Delete User",
            "3. View User File",
            "NAV: Go Back",
        ],
        None,
    );

    match choice1 {
        // listing profiles
        0 => {
            page_header("Developer Mode - User Manager", HeaderSubtext::None);

            for profile_string in &profiles {
                println!("- {}", profile_string);
            }

            println!();
            tui::press_enter_to_continue();

            user_manager(user);
        }

        // deleting profiles
        1 => {
            page_header("Developer Mode - User Manager", HeaderSubtext::Keyboard);

            let choice = select_from_vector(profiles.clone(), Some("Select a profile to delete"));

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

                        page_header("Developer Mode - User Manager", HeaderSubtext::None);
                        println!("\nCurrent profile successfully deleted. Logging out.");
                        tui::press_enter_to_continue();

                        crate::menus::accounts::main();
                    }

                    UserProfile::delete_from_username(profile_string);

                    page_header("Developer Mode - User Manager", HeaderSubtext::None);
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
        HeaderSubtext::None,
    );
    let choice = select_from_vector(UserProfile::list_all(), Some("Select a user to view"));

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
                        HeaderSubtext::None,
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

    page_header("Developer Mode - Bank Managert", HeaderSubtext::Keyboard);

    println!("Gold: {} Gold", user.bank.wallet);
    println!();
    println!("Account 1: {} Gold", user.bank.account1);
    println!("Account 2: {} Gold", user.bank.account2);
    println!("Account 3: {} Gold", user.bank.account3);
    println!("Account 4: {} Gold\n", user.bank.account4);

    let account_choice = selector(
        &[
            "Wallet",
            "Account 1",
            "Account 2",
            "Account 3",
            "Account 4",
            "NAV: Go Back",
        ],
        None,
    );

    match account_choice {
        0 => account = BankAccount::Wallet,
        1 => account = BankAccount::Account1,
        2 => account = BankAccount::Account2,
        3 => account = BankAccount::Account3,
        4 => account = BankAccount::Account4,
        5 => main(user),
        _ => out_of_bounds(None),
    }

    let calculation = generic_calculator::<usize>();

    let result: Result<(), &str> = match calculation {
        Operation::Add(_) => user.bank.arithmetic(account, calculation),
        Operation::Subtract(_) => user.bank.arithmetic(account, calculation),
        Operation::Multiply(_) => user.bank.arithmetic(account, calculation),
        Operation::Divide(_) => user.bank.arithmetic(account, calculation),
        Operation::None => Err(""),
    };

    match result {
        Ok(_) => {
            println!("\nOperation successful.");
            press_enter_to_continue();
            bank_manager(user);
        }
        Err(_) => {
            bank_manager(user);
        }
    }
}

fn xp_manager(user: &mut UserProfile) {
    page_header("Developer Mode - XP Manager", HeaderSubtext::Keyboard);

    let xp_category = selector(
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
        7 => main(user),
        _ => out_of_bounds(None),
    };

    let current_xp = user.xp.get(xp_type);
    let xp_title: &str = match xp_type {
        XPType::Combat => "Combat",
        XPType::Fishing => "Fishing",
        XPType::Cooking => "Cooking",
        XPType::Woodcutting => "Woodcutting",
        XPType::Mining => "Mining",
        XPType::Smithing => "Smithing",
        XPType::Thieving => "Thieving",
    };

    println!("{}: {}xp\n", xp_title, current_xp);

    let calculation = generic_calculator::<usize>();

    let result: Result<(), &str> = match calculation {
        Operation::Add(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::Subtract(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::Multiply(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::Divide(_) => user.xp.arithmetic(xp_type, calculation),
        Operation::None => Err(""),
    };

    match result {
        Ok(_) => {
            println!("\nOperation successful.");
            press_enter_to_continue();
            xp_manager(user);
        }
        Err(_) => {
            xp_manager(user);
        }
    }
}

fn inventory_manager(user: &mut UserProfile) {
    page_header("Developer Mode - Inventory Manager", HeaderSubtext::None);

    let manager_option = selector(
        &["1. Items", "2. Weapons", "3. Armor", "NAV: Go Back"],
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
        HeaderSubtext::None,
    );

    // TODO: Items Manager

    inventory_manager(user);
}

fn weapons_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Weapons",
        HeaderSubtext::None,
    );

    // TODO: Weapons Manager

    inventory_manager(user);
}

fn armor_manager(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Inventory Manager - Armor",
        HeaderSubtext::None,
    );

    // TODO: Armor Manager

    inventory_manager(user);
}
