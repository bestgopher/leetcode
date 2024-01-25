#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1),
        13
    );
}

struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter()
            .enumerate()
            .filter(|(x, _)| x.count_ones() as i32 == k)
            .map(|(_, x)| x)
            .sum()
    }
}
