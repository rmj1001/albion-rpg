use crate::lib::{
    input::{prompt_arrow, select_from_str_array},
    messages::{self, *},
    tui::{page_header, HeaderSubtext},
};

use crate::user::{bank::*, profile::UserProfile};

pub fn main(user: &mut UserProfile) {
    let mut account: BankAccount = BankAccount::Account1;

    page_header("The Bank", HeaderSubtext::Keyboard);

    println!();
    user.bank.print_table();

    let option = select_from_str_array(&["Deposit", "Withdraw", "NAV: Go Back"], None);

    if option == 2 {
        crate::menus::game_menu::main(user);
    }

    let account_choice = select_from_str_array(
        &[
            "Account 1",
            "Account 2",
            "Account 3",
            "Account 4",
            "NAV: Cancel",
        ],
        None,
    );

    match account_choice {
        0 => account = BankAccount::Account1,
        1 => account = BankAccount::Account2,
        2 => account = BankAccount::Account3,
        3 => account = BankAccount::Account4,
        4 => main(user),
        _ => out_of_bounds(None),
    }

    let amount_result = prompt_arrow("Amount").parse::<usize>();
    let mut amount: usize = 0;

    match amount_result {
        Ok(number) => amount = number,
        Err(_) => {
            messages::invalid_input(None, None, true);
            main(user);
        }
    }

    let mut bank_result: Result<(), &str> = Err("Unitialized");

    match option {
        // Deposit
        0 => bank_result = Bank::deposit(user, account, amount, false),
        // Withdrawal
        1 => bank_result = Bank::withdraw(user, account, amount, false),
        2 => main(user),
        _ => out_of_bounds(None),
    }

    match bank_result {
        Ok(_) => {
            success();
            main(user);
        }

        Err(message) => {
            failure(message);
            main(user);
        }
    }
}
