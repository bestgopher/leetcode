#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    /// 和的范围为n - 6n
    pub fn dices_probability(n: i32) -> Vec<f64> {
        if n == 1 {
            return vec![1.0 / 6.0; 6];
        }

        let mut r = vec![0f64; (5 * n + 1) as usize];

        let r1 = Self::dices_probability(n - 1);

        for i in 1..=6 {
            // index代表n-1个骰子的和，index=0的和为n-1
            for (index, &value) in r1.iter().enumerate() {
                r[(index + (n - 1) as usize) + i - n as usize] += value * 1.0 / 6.0;
            }
        }

        r
    }
}
