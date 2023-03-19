#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    )
}

struct Solution;

impl Solution {
    /// 单调递减栈，保存下标
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            while !stack.is_empty() {
                if temperatures[stack[stack.len() - 1]] < temperatures[i] {
                    ans[stack[stack.len() - 1]] = (i - stack[stack.len() - 1]) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(i);
        }

        ans
    }
}
