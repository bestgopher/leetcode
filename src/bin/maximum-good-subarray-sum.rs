#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut hash = std::collections::HashMap::new();
        let mut sum = 0i64;
        let mut result = i64::MIN;
        for x in nums {
            if let Some(&s) = hash.get(&(x + k)) {
                result = result.max(sum + x as i64 - s);
            }

            if let Some(&s) = hash.get(&(x - k)) {
                result = result.max(sum + x as i64 - s);
            }

            match hash.get(&x) {
                Some(&s) if s < sum => {}
                _ => {
                    hash.insert(x, sum);
                }
            }

            sum += x as i64;
        }

        if result == i64::MIN {
            0
        } else {
            result
        }
    }
}
