#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut nums = nums;
        nums.sort();

        let (mut s1, mut s2) = (0, 1); // 双指针
        let mut result = 0;

        for i in 1..nums.len() {
            result += Self::search(target - nums[i], &nums[..i]);
        }

        result
    }

    fn search(target: i32, nums: &[i32]) -> i32 {
        let (mut start, mut end) = (0, nums.len());

        while start < end {
            let middle = start + (end - start) / 2;
            if nums[middle] < target {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        end as i32
    }
}
