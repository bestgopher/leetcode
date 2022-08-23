#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_straight1(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort();

        let mut zero_num = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                zero_num += 1;
            } else {
                if i > 0 && nums[i - 1] != 0 {
                    if nums[i] - nums[i - 1] > 0 {
                        zero_num -= (nums[i] - nums[i - 1] - 1);
                    } else {
                        return false;
                    }

                    if zero_num < 0 {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn is_straight(nums: Vec<i32>) -> bool {
        let mut min = std::i32::MAX;
        let mut zero = 0; // 0 的数量
        let set = nums
            .into_iter()
            .map(|x| {
                if x == 0 {
                    zero += 1;
                } else if x < min {
                    min = x;
                }

                x
            })
            .filter(|x| *x != 0)
            .collect::<std::collections::HashSet<_>>();

        if zero == 5 {
            return true;
        }

        for i in 0..5 {
            if set.contains(&(i + min)) {
                continue;
            } else {
                if zero > 0 {
                    zero -= 1;
                } else {
                    return false;
                }
            }
        }

        true
    }
}
