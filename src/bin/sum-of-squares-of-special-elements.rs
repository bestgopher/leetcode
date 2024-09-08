#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.into_iter()
            .enumerate()
            .filter_map(|(x, y)| if n % (x + 1) == 0 { Some(y * y) } else { None })
            .sum()
    }
}
