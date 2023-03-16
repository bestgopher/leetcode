#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 逆波兰表达式配合栈
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for i in tokens {
            match &*i {
                "+" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x + y);
                }
                "-" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(y - x);
                }
                "*" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x * y);
                }
                "/" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(y / x);
                }
                s => {
                    stack.push(s.parse().unwrap());
                }
            }
        }

        stack.pop().unwrap()
    }
}
