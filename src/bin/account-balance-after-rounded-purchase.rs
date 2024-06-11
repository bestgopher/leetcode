#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let a = purchase_amount % 10;
        100 - if a >= 5 {
            purchase_amount + (10 - a)
        } else {
            purchase_amount - a
        }
    }
}
