#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let (mut a1, mut a2) = (prices[0].min(prices[1]), prices[1].max(prices[0]));

        for &i in prices[2..].iter() {
            if i <= a1 {
                a2 = a1;
                a1 = i
            } else if i > a1 && i < a2 {
                a2 = i;
            }
        }

        if a1 + a2 > money {
            money
        } else {
            money - a1 - a2
        }
    }
}
