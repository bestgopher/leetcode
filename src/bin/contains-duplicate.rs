#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash = std::collections::HashSet::new();

        for i in nums.into_iter() {
            if hash.contains(&i) {
                return true;
            }

            hash.insert(i);
        }

        false
    }
}
