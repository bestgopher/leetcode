#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 普通法
    /// 排序，挨个比较
    pub fn find_unsorted_subarray1(nums: Vec<i32>) -> i32 {
        let mut n = nums.clone();
        n.sort();
        let (mut start, mut end) = (-1, -1);

        for i in 0..nums.len() {
            if nums[i] != n[i] {
                if start == -1 {
                    start = i as i32;
                } else {
                    end = i as i32;
                }
            }
        }

        if start == -1 {
            0
        } else {
            end - start + 1
        }
    }

    // pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    //     let mut start = 0;
    //     let mut end = nums.len() - 1;

    //     for i in 1..nums.len() {
    //         if nums[i] > nums[i - 1] {
    //             start = i;
    //         } else {
    //             break;
    //         }
    //     }

    //     for i in (0..nums.len() - 1).rev() {
    //         if nums[i] < nums[i + 1] {
    //             end = i;
    //         } else {
    //             break;
    //         }
    //     }

    //     for i in start..end {
    //         while nums[i] < nums[start] {
    //             start -= 1;
    //         }

    //         while nums[i] > nums[end] {
    //             end += 1;
    //         }
    //     }

    //     (end - start + 1) as i32
    // }
}
