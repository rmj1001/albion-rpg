use crate::{
    utils::{
        input::{prompt_arrow, select_from_str_array},
        messages::{self, *},
        tui::{page_header, HeaderSubtext},
    },
    MiscError,
};

use crate::data::{inventory::bank::*, player::Player};

pub fn main(player: &mut Player) {
    let mut account: BankAccount = BankAccount::Account1;

    page_header("The Bank", HeaderSubtext::Keyboard);

    println!();
    player.bank.table();

    let option = select_from_str_array(&["1. Deposit", "2. Withdraw", "NAV: Go Back"], None);

    if option == 2 {
        crate::menus::game_menu::main(player);
    }

    let account_choice = select_from_str_array(
        &[
            "1. Account 1",
            "2. Account 2",
            "3. Account 3",
            "4. Account 4",
            "NAV: Cancel",
        ],
        None,
    );

    match account_choice {
        0 => account = BankAccount::Account1,
        1 => account = BankAccount::Account2,
        2 => account = BankAccount::Account3,
        3 => account = BankAccount::Account4,
        4 => main(player),
        _ => out_of_bounds(),
    }

    let amount_result = prompt_arrow("Amount").parse::<usize>();
    let mut amount: usize = 0;

    match amount_result {
        Ok(number) => amount = number,
        Err(_) => {
            messages::invalid_input(None, None, true);
            main(player);
        }
    }

    let mut bank_result: crate::Result<()> = Err(MiscError::Custom("Uninitialized").boxed());

    match option {
        // Deposit
        0 => bank_result = Bank::deposit(player, account, amount, false),
        // Withdrawal
        1 => bank_result = Bank::withdraw(player, account, amount, false),
        2 => main(player),
        _ => out_of_bounds(),
    }

    match bank_result {
        Ok(_) => {
            success();
            main(player);
        }

        Err(message) => {
            message.failure();
            main(player);
        }
    }
}
