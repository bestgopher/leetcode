#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut r, mut min) = (0, prices[0]);

        for i in 1..prices.len() {
            r = r.max(prices[i] - min);
            min = min.min(prices[i]);
        }

        r
    }
}
