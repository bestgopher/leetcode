#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for i in operations.iter() {
            match i.as_str() {
                "C" => _ = stack.pop(),
                "D" => {
                    stack.push(stack[stack.len() - 1] * 2);
                }
                "+" => {
                    let x = stack[stack.len() - 1];
                    let y = stack[stack.len() - 2];
                    stack.push(x + y);
                }

                m => stack.push(m.parse().unwrap()),
            }
        }

        stack.into_iter().sum()
    }
}
