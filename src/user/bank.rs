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
    pub fn arithmetic(
        &mut self,
        account_flag: BankAccount,
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
            Operation::None => {
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
            return Err("There is not enough gold in the account");
        }

        if !add_only {
            user.bank.wallet -= amount;
        }

        user.bank.arithmetic(account_flag, Operation::Add(amount))
    }

    pub fn withdraw(
        user: &mut UserProfile,
        account_flag: BankAccount,
        amount: usize,
        subtract_only: bool,
    ) -> Result<(), &str> {
        if !subtract_only {
            user.bank.wallet += amount;
        }

        user.bank
            .arithmetic(account_flag, Operation::Subtract(amount))
    }
}
