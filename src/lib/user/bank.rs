use super::profile::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum BankAccount {
    Account1,
    Account2,
    Account3,
    Account4,
}

#[derive(Serialize, Deserialize)]
pub enum BankResult {
    Ok,
    Error(&'static str),
}

#[derive(Serialize, Deserialize)]
pub struct Bank {
    pub account1: u32,
    pub account2: u32,
    pub account3: u32,
    pub account4: u32,
}

impl Bank {
    pub fn deposit(
        user: &mut UserProfile,
        account_flag: BankAccount,
        amount: u32,
        add_only: bool,
    ) -> BankResult {
        if !add_only && user.gold < amount {
            return BankResult::Error("There is not enough gold in the account");
        }

        if !add_only {
            user.gold -= amount;
        }

        match account_flag {
            BankAccount::Account1 => {
                user.bank.account1 += amount;
            }
            BankAccount::Account2 => {
                user.bank.account2 += amount;
            }
            BankAccount::Account3 => {
                user.bank.account3 += amount;
            }
            BankAccount::Account4 => {
                user.bank.account4 += amount;
            }
        }

        BankResult::Ok
    }

    pub fn withdraw(
        user: &mut UserProfile,
        account_flag: BankAccount,
        amount: u32,
        withdraw_only: bool,
    ) -> BankResult {
        match account_flag {
            BankAccount::Account1 => {
                if user.bank.account1 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    user.bank.account1 -= amount;
                }
            }
            BankAccount::Account2 => {
                if user.bank.account2 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    user.bank.account2 -= amount;
                }
            }
            BankAccount::Account3 => {
                if user.bank.account3 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    user.bank.account3 -= amount;
                }
            }
            BankAccount::Account4 => {
                if user.bank.account4 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    user.bank.account4 -= amount;
                }
            }
        }

        if !withdraw_only {
            user.gold += amount;
        }

        BankResult::Ok
    }
}
