use crate::lib::{math::Operation, tui::press_enter_to_continue};

use super::profile::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BankAccount {
    Wallet,
    Account1,
    Account2,
    Account3,
    Account4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank {
    pub wallet: usize,
    pub account1: usize,
    pub account2: usize,
    pub account3: usize,
    pub account4: usize,
}

impl Bank {
    pub fn balance(user: &UserProfile, account: &BankAccount) -> usize {
        match account {
            BankAccount::Wallet => user.bank.wallet,
            BankAccount::Account1 => user.bank.account1,
            BankAccount::Account2 => user.bank.account2,
            BankAccount::Account3 => user.bank.account3,
            BankAccount::Account4 => user.bank.account4,
        }
    }

    pub fn print_table(&self) {
        let accounts = [
            "Account,Balance".to_string(),
            format!("Wallet,{}", self.wallet),
            format!("Account 1,{}", self.account1),
            format!("Account 2,{}", self.account2),
            format!("Account 3,{}", self.account3),
            format!("Account 4,{}", self.account4),
        ]
        .join("\n");

        let table = csv_to_table::iter::from_reader(accounts.as_bytes()).to_string();

        println!("{}\n", table);
    }

    pub fn arithmetic(
        &mut self,
        account_flag: &BankAccount,
        operation: Operation<usize>,
    ) -> Result<(), &str> {
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
                    Err("The amount is greater than the account balance.")
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
                println!("\nCancelling.");
                press_enter_to_continue();
                Ok(())
            }
            Operation::Invalid => {
                println!("\nOperation failed: Invalid Operator");
                press_enter_to_continue();
                Err("Operation failed: Invalid Operator")
            }
        }
    }

    pub fn deposit(
        user: &mut UserProfile,
        account_flag: BankAccount,
        amount: usize,
        add_only: bool,
    ) -> Result<(), &str> {
        if !add_only && user.bank.wallet < amount {
            return Err("You do not have enough gold in your wallet.");
        }

        if !add_only {
            user.bank.wallet -= amount;
        }

        user.bank.arithmetic(&account_flag, Operation::Add(amount))
    }

    pub fn withdraw(
        user: &mut UserProfile,
        account_flag: BankAccount,
        amount: usize,
        subtract_only: bool,
    ) -> Result<(), &str> {
        let account_balance: usize = Bank::balance(user, &account_flag);

        if account_balance >= amount && !subtract_only {
            user.bank.wallet += amount;
        }

        let withdraw_result = user
            .bank
            .arithmetic(&account_flag, Operation::Subtract(amount));

        withdraw_result.to_owned()
    }
}
