#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
}

struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().map(|x| *x).sum();

        let mut left = 0;
        for i in 0..nums.len() {
            if left == sum - left - nums[i] {
                return i as i32;
            }

            left += nums[i];
        }

        -1
    }
}
