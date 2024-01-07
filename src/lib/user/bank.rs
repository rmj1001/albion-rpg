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
        &mut self,
        user: &mut UserProfile,
        account_flag: BankAccount,
        amount: u32,
    ) -> BankResult {
        if user.gold < amount {
            return BankResult::Error("There is not enough gold in the account");
        }

        user.gold -= amount;

        match account_flag {
            BankAccount::Account1 => {
                self.account1 += amount;
            }
            BankAccount::Account2 => {
                self.account2 += amount;
            }
            BankAccount::Account3 => {
                self.account3 += amount;
            }
            BankAccount::Account4 => {
                self.account4 += amount;
            }
        }

        BankResult::Ok
    }

    pub fn withdraw(
        &mut self,
        user: &mut UserProfile,
        account_flag: BankAccount,
        amount: u32,
    ) -> BankResult {
        match account_flag {
            BankAccount::Account1 => {
                if self.account1 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    self.account1 -= amount;
                }
            }
            BankAccount::Account2 => {
                if self.account2 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    self.account2 -= amount;
                }
            }
            BankAccount::Account3 => {
                if self.account3 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    self.account3 -= amount;
                }
            }
            BankAccount::Account4 => {
                if self.account4 < amount {
                    return BankResult::Error("There is not enough gold in the account");
                } else {
                    self.account4 -= amount;
                }
            }
        }

        user.gold += amount;
        BankResult::Ok
    }
}
