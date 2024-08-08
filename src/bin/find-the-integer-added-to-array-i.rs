#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut min1, mut min2) = (nums1[0], nums2[0]);

        for i in 1..nums1.len() {
            min1 = min1.min(nums1[i]);
            min2 = min2.min(nums2[i]);
        }

        min2 - min1
    }
}
