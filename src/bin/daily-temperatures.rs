#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    /// 单调栈保存单调递减的温度，
    /// 遍历到一个温度时，弹出单调栈顶的元素，如果当前温度比栈顶的温度高，则就是第一次出现大于栈顶的温度，直接下标求差即为相差的天数。
    /// 然后把当前温度入栈。
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            while !stack.is_empty() {
                if temperatures[stack[stack.len() - 1]] < temperatures[i] {
                    ans[stack[stack.len() - 1]] = (i - (stack[stack.len() - 1])) as i32;
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
