#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }

        // i: 第一个0的下标
        let mut i = 0;

        for j in 0..nums.len() {
            if nums[j] != 0 {
                while i < j {
                    if nums[i] == 0 {
                        nums.swap(i, j);
                        break;
                    }
                    i += 1;
                }
            }
        }

        nums
    }
}
