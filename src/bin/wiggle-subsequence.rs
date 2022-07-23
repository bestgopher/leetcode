#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        // 当长度小于等于2时，就可能为1或者2，结果取决与长度
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let (mut down, mut up) = (1, 1);

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                up = down + 1;
            } else if nums[i] < nums[i - 1] {
                down = up + 1;
            }
        }

        down.max(up)
    }
}
