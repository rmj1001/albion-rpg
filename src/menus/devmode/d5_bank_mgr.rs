use crate::{
    misc::{
        input::select_from_str_array,
        math::{generic_calculator, Operation},
        messages::*,
        tui::{page_header, press_enter_to_continue, HeaderSubtext},
    },
    user::{bank::BankAccount, profile::UserProfile},
};

pub fn main(user: &mut UserProfile) {
    let mut account: BankAccount = BankAccount::Account1;

    page_header("Developer Mode - Bank Managert", HeaderSubtext::Keyboard);

    user.bank.print_table();

    let account_choice = select_from_str_array(
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
        5 => super::d1_developer_menu::main(user),
        _ => out_of_bounds(),
    }

    let calculation = generic_calculator::<usize>();

    // Return early if the operation was cancelled.
    if let Operation::Cancel = calculation {
        cancelling();
        press_enter_to_continue();
        main(user);
    }

    let result: Result<(), &str> = match calculation {
        Operation::Add(_) => user.bank.arithmetic(&account, calculation),
        Operation::Subtract(_) => user.bank.arithmetic(&account, calculation),
        Operation::Multiply(_) => user.bank.arithmetic(&account, calculation),
        Operation::Divide(_) => user.bank.arithmetic(&account, calculation),
        Operation::Cancel => Ok(()),
        Operation::Invalid => Err(""),
    };

    match result {
        Ok(_) => {
            success();
            main(user);
        }
        Err(_) => {
            main(user);
        }
    }
}
