#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
    assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
    assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
}

struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let (mut m1, mut m2) = (vec![0; nums.len()], vec![0; nums.len()]);
        for i in 0..nums.len() {
            if i == 0 {
                m1[i] = nums[i];
            } else {
                m1[i] = m1[i - 1].min(nums[i]);
            }
        }

        for i in (0..nums.len()).rev() {
            if i == nums.len() - 1 {
                m2[i] = nums[i];
            } else {
                m2[i] = m2[i + 1].min(nums[i]);
            }
        }

        let mut result = i32::MAX;
        for i in 1..nums.len() - 1 {
            if m1[i - 1] < nums[i] && nums[i] > m2[i + 1] {
                result = result.min(m1[i - 1] + m2[i + 1] + nums[i])
            }
        }

        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}
