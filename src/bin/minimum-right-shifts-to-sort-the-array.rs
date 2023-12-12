#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut f = false;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                if f {
                    return -1;
                }

                f = true;
            }

            if f {
                r += 1;
            }
        }

        if f {
            if nums[0] >= nums[nums.len() - 1] {
                r
            } else {
                -1
            }
        } else {
            0
        }
    }
}
