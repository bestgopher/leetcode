#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    /// 单调递减栈
    /// 找出以0下标为起点的单调递减元素的下标，放到栈中
    /// 然后从后往前比较出最大值
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut stack = vec![];
        for i in 0..a.len() {
            if stack.is_empty() || a[i] <= a[*stack.last().unwrap()] {
                stack.push(i);
            }
        }

        let mut result = 0;

        for i in (0..a.len()).rev() {
            if stack.is_empty() {
                break;
            }

            while !stack.is_empty() && a[i] >= a[*stack.last().unwrap()] {
                let r = i - stack.pop().unwrap();
                if r >= result {
                    result = r;
                }
            }
        }

        result as i32
    }
}
