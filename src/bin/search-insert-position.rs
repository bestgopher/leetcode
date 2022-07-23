#![allow(dead_code, unused, unused_variables)]

fn main() {
    // assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    // assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
    // assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    // assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    // assert_eq!(2, Solution::search_insert(vec![1, 2, 4, 6, 7], 3));
    // assert_eq!(0, Solution::search_insert(vec![1, 3, 5], 1));
    // assert_eq!(0, Solution::search_insert(vec![1, 3], 0));
    assert_eq!(1, Solution::search_insert(vec![1, 3], 2));
}

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);
        while start < end {
            let middle = (start + end) >> 1;
            if nums[middle] == target {
                break;
            } else if nums[middle] < target {
                start = middle + 1;
            } else {
                end = if middle > 1 { middle - 1 } else { 0 };
            }
        }
        let middle = (start + end) >> 1;
        if nums[middle] > target {
            if middle > 0 {
                middle as i32
            } else {
                0
            }
        } else if nums[middle] < target {
            middle as i32 + 1
        } else {
            middle as i32
        }
    }
}
