#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut v = vec![];

        for i in tokens.into_iter() {
            if let Ok(i) = i.parse::<i32>() {
                v.push(i);
            } else {
                let (x1, x2) = (v.pop().unwrap(), v.pop().unwrap());

                let x = match i.as_str() {
                    "+" => x1 + x2,
                    "-" => x2 - x1,
                    "*" => x1 * x2,
                    "/" => x2 / x1,
                    _ => 0,
                };

                v.push(x);
            }
        }
        v[0]
    }
}
