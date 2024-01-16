use crate::lib::{
    input::{self, out_of_bounds, prompt_input, selector},
    tui::{page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::user::{bank::*, profile::UserProfile};

pub fn main(user: &mut UserProfile) {
    let mut account: BankAccount = BankAccount::Account1;

    page_header("The Bank", HeaderSubtext::Keyboard);
    println!("Gold: {} Gold", user.bank.wallet);
    println!();
    println!("Account 1: {} Gold", user.bank.account1);
    println!("Account 2: {} Gold", user.bank.account2);
    println!("Account 3: {} Gold", user.bank.account3);
    println!("Account 4: {} Gold\n", user.bank.account4);

    let account_choice = selector(
        &[
            "Account 1",
            "Account 2",
            "Account 3",
            "Account 4",
            "NAV: Go Back",
        ],
        None,
    );

    match account_choice {
        0 => account = BankAccount::Account1,
        1 => account = BankAccount::Account2,
        2 => account = BankAccount::Account3,
        3 => account = BankAccount::Account4,
        4 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(None),
    }

    let option = selector(&["Deposit", "Withdraw", "NAV: Cancel"], None);

    if option == 2 {
        main(user);
    }

    let amount_result = prompt_input("Amount").parse::<usize>();
    let mut amount: usize = 0;

    match amount_result {
        Ok(number) => amount = number,
        Err(_) => {
            input::invalid_input(None, None, true);
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
            println!("\nOperation successful.");
            press_enter_to_continue();
            main(user);
        }

        Err(message) => {
            println!("\n{}", message);
            press_enter_to_continue();
            main(user);
        }
    }
}
