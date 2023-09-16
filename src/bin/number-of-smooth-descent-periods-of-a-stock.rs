#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut r = 0;
        let mut current = prices[0];
        let mut current_num = 1;

        for i in 1..prices.len() {
            if current - 1 == prices[i] {
                current_num += 1;
            } else {
                r += (current_num + 1) * current_num / 2;
                current_num = 1;
            }
            current = prices[i];
        }
        r += (current_num + 1) * current_num / 2;
        r
    }
}
