#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
    assert_eq!(true, Solution::can_jump(vec![0]));
}

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_position = 0;

        for i in 0..nums.len() {
            if i > max_position {
                return false;
            }

            max_position = max_position.max(i + nums[i] as usize);
        }

        true
    }
}
