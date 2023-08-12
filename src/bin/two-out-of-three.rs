#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::collections::HashSet;

fn main() {}

struct Solution;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut hash1: HashSet<_> = nums1.iter().collect();
        let mut hash2 = HashSet::new();
        let mut hash3 = HashSet::new();

        let mut result = vec![];

        for i in nums2.iter() {
            if hash1.contains(i) && !hash2.contains(i) {
                result.push(*i);
            }
            hash2.insert(*i);
        }

        for i in nums3.iter() {
            if ((hash1.contains(i) && !hash2.contains(i))
                || (!hash1.contains(i) && hash2.contains(i)))
                && !hash3.contains(i)
            {
                result.push(*i);
            }
            hash3.insert(*i);
        }

        result
    }
}
