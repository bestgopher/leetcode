#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        Self::dp(&mut std::collections::HashMap::new(), &nums, target)
    }

    pub fn dp(hash: &mut std::collections::HashMap<i32, i32>, nums: &[i32], target: i32) -> i32 {
        if let Some(x) = hash.get(&target) {
            return *x;
        }

        if target == 0 {
            return 1;
        } else if target < 0 {
            return 0;
        }

        let mut result = 0;
        for &i in nums {
            result += Self::dp(hash, nums, target - i);
        }

        hash.insert(target, result);

        result
    }
}
