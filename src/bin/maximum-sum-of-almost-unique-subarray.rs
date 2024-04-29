#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::max_sum(vec![2, 6, 7, 3, 1, 7], 3, 4), 18);
    assert_eq!(Solution::max_sum(vec![5, 9, 9, 2, 4, 5, 4], 1, 3), 23);
    assert_eq!(Solution::max_sum(vec![1, 2, 1, 2, 1, 2, 1], 3, 3), 0);
}

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let mut hash = std::collections::HashMap::new();
        let mut result = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            hash.entry(nums[i]).and_modify(|x| *x += 1).or_insert(1);
            if i < k as usize {
                sum += nums[i] as i64;
            } else {
                sum += (nums[i] - nums[i - k as usize]) as i64;
                let c = if let Some(&x) = hash.get(&nums[i - k as usize]) {
                    x
                } else {
                    -1
                };

                if c == 1 {
                    hash.remove(&nums[i - k as usize]);
                } else if c > 1 {
                    hash.insert(nums[i - k as usize], c - 1);
                }
            }

            if hash.len() >= m as usize {
                result = result.max(sum);
            }
        }

        result
    }
}
