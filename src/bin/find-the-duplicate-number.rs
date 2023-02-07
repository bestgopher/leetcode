#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![2, 2, 2]), 2);
}

struct Solution;

impl Solution {
    /// 环形列表, 找到第一个入环的值
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut slow, mut fast) = (0, 0);

        while fast < nums.len() {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if nums[slow] == nums[fast] {
                break;
            }
        }

        let mut slow = 0;
        while nums[slow] != nums[fast] {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }

        nums[slow]
    }
}
