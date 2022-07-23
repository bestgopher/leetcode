#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!(
        "{:?}",
        Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
    )
}

struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let a = nums.len() + 1;

        for i in 0..nums.len() {
            let index = (nums[i] as usize % a) - 1usize;
            nums[index] += a as i32;
        }

        nums.into_iter()
            .enumerate()
            .filter(|&(i, x)| x / (a as i32) == 2)
            .map(|(i, x)| i as i32 + 1)
            .collect()
    }
}
