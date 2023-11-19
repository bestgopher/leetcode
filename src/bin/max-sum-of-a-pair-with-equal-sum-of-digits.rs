#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let mut result = -1;
        for i in nums {
            let s = Self::sum(i);
            if !hash.contains_key(&s) {
                hash.insert(s, i);
                continue;
            }

            let old = hash[&s];
            result = result.max(old + i);
            if old < i {
                hash.insert(s, i);
            }
        }

        result
    }

    pub fn sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }

        sum
    }
}
