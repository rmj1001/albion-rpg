use crate::data::player::Player;
use crate::prelude::{
    csv_table, invalid_input, page_header, prompt, select, success, unreachable, Instructions, InventoryError,
    MiscError, Result,
};
use serde::{Deserialize, Serialize};

pub enum Account {
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
        csv_table(&[
            "Account,Balance".to_string(),
            format!("Wallet,{}", self.wallet),
            format!("Account 1,{}", self.account1),
            format!("Account 2,{}", self.account2),
            format!("Account 3,{}", self.account3),
            format!("Account 4,{}", self.account4),
        ]);
    }

    pub fn balance(player: &Player, account: &Account) -> usize {
        match account {
            Account::Wallet => player.bank.wallet,
            Account::Account1 => player.bank.account1,
            Account::Account2 => player.bank.account2,
            Account::Account3 => player.bank.account3,
            Account::Account4 => player.bank.account4,
        }
    }

    pub fn account<'a>(player: &'a mut Player, account: &Account) -> &'a mut usize {
        match account {
            Account::Account1 => &mut player.bank.account1,
            Account::Account2 => &mut player.bank.account2,
            Account::Account3 => &mut player.bank.account3,
            Account::Account4 => &mut player.bank.account4,
            Account::Wallet => &mut player.bank.wallet,
        }
    }

    pub fn deposit(player: &mut Player, account_flag: &Account, amount: usize, use_wallet: bool) -> Result<()> {
        let wallet_balance: usize = Self::balance(player, &Account::Wallet);

        if use_wallet && wallet_balance < amount {
            return Err(Box::new(InventoryError::NotEnoughGold));
        }

        if use_wallet {
            player.bank.wallet -= amount;
        }

        *Self::account(player, account_flag) += amount;
        Ok(())
    }

    pub fn withdraw(player: &mut Player, account_flag: &Account, amount: usize, use_wallet: bool) -> Result<()> {
        let account_balance: usize = Self::balance(player, account_flag);

        if account_balance < amount {
            return Err(Box::new(InventoryError::NotEnoughGold));
        }

        if use_wallet {
            player.bank.wallet += amount;
        }

        *Self::account(player, account_flag) -= amount;
        Ok(())
    }

    pub fn net_worth(&self) -> usize {
        self.account1 + self.account2 + self.account3 + self.account4 + self.wallet
    }

    pub fn menu(player: &mut Player, developer_mode: bool) {
        page_header("The Bank", &Instructions::Keyboard);

        println!();
        player.bank.table();

        let option = select(&["1. Deposit", "2. Withdraw", "NAV: Go Back"], None);

        // Go to the main game menu
        if option == 2 {
            if developer_mode {
                crate::menus::devmode::d1_developer_menu::main(player);
            } else {
                crate::menus::game_menu::main(player);
            }
        }

        let account_choice = if developer_mode {
            select(
                &[
                    "1. Wallet",
                    "2. Account 1",
                    "3. Account 2",
                    "4. Account 3",
                    "5. Account 4",
                    "NAV: Cancel",
                ],
                None,
            )
        } else {
            select(
                &[
                    "1. Account 1",
                    "2. Account 2",
                    "3. Account 3",
                    "4. Account 4",
                    "NAV: Cancel",
                ],
                None,
            )
        };

        let mut account: Account = Account::Account1;

        if developer_mode {
            match account_choice {
                0 => account = Account::Wallet,
                1 => account = Account::Account1,
                2 => account = Account::Account2,
                3 => account = Account::Account3,
                4 => account = Account::Account4,
                5 => Self::menu(player, developer_mode),
                _ => unreachable(),
            }
        } else {
            match account_choice {
                0 => account = Account::Account1,
                1 => account = Account::Account2,
                2 => account = Account::Account3,
                3 => account = Account::Account4,
                4 => Self::menu(player, developer_mode),
                _ => unreachable(),
            }
        }

        let amount_result = prompt("Amount").parse::<usize>();
        let mut amount: usize = 0;

        if let Ok(number) = amount_result {
            amount = number;
        } else {
            invalid_input(None, None, true);
            Self::menu(player, developer_mode);
        }

        let mut bank_result: Result<()> = Err(Box::new(MiscError::Custom("Uninitialized")));

        // If developer mode is on then don't use the wallet, and vice versa.
        let use_wallet: bool = !developer_mode;

        match option {
            // Deposit
            0 => bank_result = Self::deposit(player, &account, amount, use_wallet),

            // Withdrawal
            1 => bank_result = Self::withdraw(player, &account, amount, use_wallet),

            // The "Go Back" option was already handled.
            _ => unreachable(),
        }

        match bank_result {
            Ok(()) => success(None),
            Err(message) => message.print(true),
        }

        Self::menu(player, developer_mode);
    }
}
