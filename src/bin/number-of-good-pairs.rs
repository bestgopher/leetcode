#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let mut result = 0;
        for i in nums {
            if let Some(&x) = hash.get(&i) {
                result += x;
            }

            hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }
        result
    }
}
