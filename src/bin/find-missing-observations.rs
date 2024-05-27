#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut m = rolls.len() as i32;
        let total = mean * (m + n);
        let rolls_sum: i32 = rolls.into_iter().sum();
        let mut n_sum = total - rolls_sum;
        let mut result = Vec::with_capacity(n as usize);

        if n_sum > n * 6 || n_sum < n {
            return vec![];
        }

        for i in 1..=n {
            match n_sum - (n - i) {
                x if x >= 0 && x <= 5 => {
                    result.push(x);
                    n_sum -= x;
                }

                _ => {
                    result.push(6);
                    n_sum -= 6;
                }
            }
        }

        result
    }
}
