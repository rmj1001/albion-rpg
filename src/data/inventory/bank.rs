use crate::utils::{math::Operation, messages::*, tui::table_from_csv};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            account1: 0,
            account2: 0,
            account3: 0,
            account4: 0,
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

    pub fn arithmetic(&mut self, account_flag: &BankAccount, operation: Operation<usize>) -> crate::Result<()> {
        let account = match account_flag {
            BankAccount::Account1 => &mut self.account1,
            BankAccount::Account2 => &mut self.account2,
            BankAccount::Account3 => &mut self.account3,
            BankAccount::Account4 => &mut self.account4,
            BankAccount::Wallet => &mut self.wallet,
        };

        match operation {
            Operation::Add(amount) => {
                *account += amount;
                Ok(())
            }

            Operation::Subtract(amount) => {
                if amount > *account {
                    Err(InventoryError::NotEnoughGold.boxed())
                } else {
                    *account -= amount;
                    Ok(())
                }
            }

            Operation::Multiply(amount) => {
                *account *= amount;
                Ok(())
            }

            Operation::Divide(amount) => {
                *account /= amount;
                Ok(())
            }
            Operation::Cancel => {
                cancelling();
                Ok(())
            }
            Operation::Invalid => {
                failure("Invalid Operator.");
                Err(MiscError::InvalidOperator.boxed())
            }
        }
    }

    pub fn deposit(player: &mut Player, account_flag: BankAccount, amount: usize, add_only: bool) -> crate::Result<()> {
        if !add_only && player.bank.wallet < amount {
            return Err(InventoryError::NotEnoughGold.boxed());
        }

        if !add_only {
            player.bank.wallet -= amount;
        }

        player.bank.arithmetic(&account_flag, Operation::Add(amount))
    }

    pub fn withdraw(
        player: &mut Player,
        account_flag: BankAccount,
        amount: usize,
        subtract_only: bool,
    ) -> crate::Result<()> {
        let account_balance: usize = Bank::balance(player, &account_flag);

        if account_balance >= amount && !subtract_only {
            player.bank.wallet += amount;
        }

        player.bank.arithmetic(&account_flag, Operation::Subtract(amount))
    }

    pub fn net_worth(&self) -> usize {
        self.account1 + self.account2 + self.account3 + self.account4 + self.wallet
    }
}
