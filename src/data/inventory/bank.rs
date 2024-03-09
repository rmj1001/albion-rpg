use crate::utils::{messages::*, tui::table_from_csv};

use crate::data::player::*;
use crate::{InventoryError, MiscError};
use serde::{Deserialize, Serialize};

pub enum BankAccount {
    Wallet,
    Account1,
    Account2,
    Account3,
    Account4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Bank {
    pub wallet: usize,
    pub account1: usize,
    pub account2: usize,
    pub account3: usize,
    pub account4: usize,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            wallet: 10,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Prints Bank information as a table using CSV formatting.
    pub fn table(&self) {
        table_from_csv(vec![
            "Account,Balance".to_string(),
            format!("Wallet,{}", self.wallet),
            format!("Account 1,{}", self.account1),
            format!("Account 2,{}", self.account2),
            format!("Account 3,{}", self.account3),
            format!("Account 4,{}", self.account4),
        ])
    }

    pub fn balance(player: &Player, account: &BankAccount) -> usize {
        match account {
            BankAccount::Wallet => player.bank.wallet,
            BankAccount::Account1 => player.bank.account1,
            BankAccount::Account2 => player.bank.account2,
            BankAccount::Account3 => player.bank.account3,
            BankAccount::Account4 => player.bank.account4,
        }
    }

    pub fn account<'a>(player: &'a mut Player, account: &BankAccount) -> &'a mut usize {
        match account {
            BankAccount::Account1 => &mut player.bank.account1,
            BankAccount::Account2 => &mut player.bank.account2,
            BankAccount::Account3 => &mut player.bank.account3,
            BankAccount::Account4 => &mut player.bank.account4,
            BankAccount::Wallet => &mut player.bank.wallet,
        }
    }

    pub fn deposit(
        player: &mut Player,
        account_flag: BankAccount,
        amount: usize,
        use_wallet: bool,
    ) -> crate::Result<()> {
        let wallet_balance: usize = Self::balance(player, &BankAccount::Wallet);

        if use_wallet && wallet_balance < amount {
            return Err(InventoryError::NotEnoughGold.boxed());
        }

        if use_wallet {
            player.bank.wallet -= amount;
        }

        *Self::account(player, &account_flag) += amount;
        Ok(())
    }

    pub fn withdraw(
        player: &mut Player,
        account_flag: BankAccount,
        amount: usize,
        use_wallet: bool,
    ) -> crate::Result<()> {
        let account_balance: usize = Self::balance(player, &account_flag);

        if account_balance < amount {
            return Err(InventoryError::NotEnoughGold.boxed());
        }

        if use_wallet {
            player.bank.wallet += amount;
        }

        *Self::account(player, &account_flag) -= amount;
        Ok(())
    }

    pub fn net_worth(&self) -> usize {
        self.account1 + self.account2 + self.account3 + self.account4 + self.wallet
    }

    pub fn menu(player: &mut Player, developer_mode: bool) {
        use crate::utils::input::{prompt_arrow, select_from_str_array};
        use crate::utils::tui::{page_header, HeaderSubtext};

        page_header("The Bank", HeaderSubtext::Keyboard);

        println!();
        player.bank.table();

        let option = select_from_str_array(&["1. Deposit", "2. Withdraw", "NAV: Go Back"], None);

        // Go to the main game menu
        if option == 2 {
            match developer_mode {
                true => crate::menus::devmode::d1_developer_menu::main(player),
                false => crate::menus::game_menu::main(player),
            }
        }

        let account_choice = match developer_mode {
            true => select_from_str_array(
                &[
                    "1. Wallet",
                    "2. Account 1",
                    "3. Account 2",
                    "4. Account 3",
                    "5. Account 4",
                    "NAV: Cancel",
                ],
                None,
            ),
            false => select_from_str_array(
                &[
                    "1. Account 1",
                    "2. Account 2",
                    "3. Account 3",
                    "4. Account 4",
                    "NAV: Cancel",
                ],
                None,
            ),
        };

        let mut account: BankAccount = BankAccount::Account1;

        match developer_mode {
            true => match account_choice {
                0 => account = BankAccount::Wallet,
                1 => account = BankAccount::Account1,
                2 => account = BankAccount::Account2,
                3 => account = BankAccount::Account3,
                4 => account = BankAccount::Account4,
                5 => Self::menu(player, developer_mode),
                _ => out_of_bounds(),
            },
            false => match account_choice {
                0 => account = BankAccount::Account1,
                1 => account = BankAccount::Account2,
                2 => account = BankAccount::Account3,
                3 => account = BankAccount::Account4,
                4 => Self::menu(player, developer_mode),
                _ => out_of_bounds(),
            },
        }

        let amount_result = prompt_arrow("Amount").parse::<usize>();
        let mut amount: usize = 0;

        match amount_result {
            Ok(number) => amount = number,
            Err(_) => {
                invalid_input(None, None, true);
                Self::menu(player, developer_mode);
            }
        }

        let mut bank_result: crate::Result<()> = Err(MiscError::Custom("Uninitialized").boxed());

        // If developer mode is on then don't use the wallet, and vice versa.
        let use_wallet: bool = !developer_mode;

        match option {
            // Deposit
            0 => bank_result = Self::deposit(player, account, amount, use_wallet),

            // Withdrawal
            1 => bank_result = Self::withdraw(player, account, amount, use_wallet),

            // The "Go Back" option was already handled.
            _ => out_of_bounds(),
        }

        match bank_result {
            Ok(_) => success(),
            Err(message) => message.failure(),
        }

        Self::menu(player, developer_mode);
    }
}
