#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut r = vec![0; amount as usize + 1];
        r[0] = 1;

        for coin in coins {
            for i in coin..=amount {
                r[i as usize] += r[(i - coin) as usize];
            }
        }

        r[amount as usize]
    }
}
