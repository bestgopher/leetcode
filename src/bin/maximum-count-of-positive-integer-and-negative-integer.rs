#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
    assert_eq!(Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
    assert_eq!(Solution::maximum_count(vec![5, 20, 66, 1314]), 4);
}

struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut index1 = nums.partition_point(|x| *x < 0);
        let mut index2 = nums.partition_point(|x| *x < 1);
        (index1 as i32).max((nums.len() - index2) as i32)
    }
}
