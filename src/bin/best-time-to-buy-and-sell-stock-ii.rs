#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut r, mut min) = (0, prices[0]);

        for i in 1..prices.len() {
            if prices[i] < prices[i - 1] {
                min = prices[i];
            } else {
                r += prices[i] - min;
                min = prices[i];
            }
        }
        r
    }
}
