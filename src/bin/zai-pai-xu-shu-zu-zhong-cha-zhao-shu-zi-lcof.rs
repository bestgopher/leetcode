#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    // 二分法
    // 找到第一个数和最后一个数的下标，相减得到数量
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len());
        let (mut left, mut right) = (0, 0);

        // 搜索右边界
        while start < end {
            let middle = (end + start) / 2;

            if nums[middle] <= target {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        right = start as i32;

        let (mut start, mut end) = (0, nums.len());

        // 搜索左边界
        while start < end {
            let middle = (end + start) / 2;

            if nums[middle] >= target {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

        left = start as i32;

        right - left
    }
}
