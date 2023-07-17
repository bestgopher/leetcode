#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let total_1: i32 = nums.iter().map(|x| *x).sum();

        let mut result = vec![0];
        let (mut left_0, mut right_1) = (0, total_1);
        let mut max = total_1;

        for (i, v) in nums.into_iter().enumerate() {
            if v == 1 {
                right_1 -= 1;
            } else if v == 0 {
                left_0 += 1;
            }

            match (left_0 + right_1).cmp(&max) {
                std::cmp::Ordering::Greater => {
                    max = left_0 + right_1;
                    result = vec![i as i32 + 1];
                }

                std::cmp::Ordering::Equal => {
                    result.push(i as i32 + 1);
                }
                _ => {}
            }
        }

        result
    }
}
