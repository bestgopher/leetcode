#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
}

struct Solution;

impl Solution {
    /// 使用单调递减栈stack。栈维护单调nums单调递减的数据下标。
    /// 因为stack单调递减，因此遍历到nums[i]时，只要stack.pop()的下标满足跳k步到i，则i的最大值只为nums[i] + nums[stack.pop_front()],
    /// 然后弹出stack后面比nums[i]小的元素，最后把i插入stack中即可。
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        stack.push_back(0usize);
        let mut nums = nums;

        for i in 1..nums.len() {
            while let Some(x) = stack.pop_front() {
                if x + k as usize >= i {
                    nums[i] += nums[x];
                    stack.push_front(x);
                    break;
                }
            }

            while let Some(index) = stack.pop_back() {
                if nums[index] > nums[i] {
                    stack.push_back(index);
                    break;
                }
            }

            stack.push_back(i);
        }

        nums.pop().unwrap()
    }
}
