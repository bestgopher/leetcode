#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(Solution::min_count(vec![4, 2, 1]), 4);
}

struct Solution;

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.into_iter().map(|x| x / 2 + x % 2).sum()
    }
}
