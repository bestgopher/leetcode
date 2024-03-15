#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1 as usize > self.balance.len() || account2 as usize > self.balance.len() {
            return false;
        }
        if self.balance[account1 as usize - 1] >= money {
            self.balance[account2 as usize - 1] += money;
            self.balance[account1 as usize - 1] -= money;
            true
        } else {
            false
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account as usize > self.balance.len() {
            return false;
        }

        self.balance[account as usize - 1] += money;
        return true;
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account as usize > self.balance.len() {
            return false;
        }

        if self.balance[account as usize - 1] >= money {
            self.balance[account as usize - 1] -= money;
            true
        } else {
            false
        }
    }
}
