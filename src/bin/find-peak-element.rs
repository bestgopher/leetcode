#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_peak_element1(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                start = i as i32;
            }

            if i + 1 < nums.len() && nums[i] > nums[i + 1] {
                break;
            }
        }

        start
    }

    /// 二分查找，当中间元素大于前一个元素时，
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);

        while start < end {
            let middle = (start + end) / 2;

            if nums[middle] > nums[middle + 1] {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

        start as i32
    }
}
