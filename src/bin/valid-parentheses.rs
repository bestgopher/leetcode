#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for i in s.chars() {
            match i {
                '(' | '{' | '[' => stack.push(i),
                _ => {
                    if let Some(x) = stack.pop() {
                        if (x == '(' && i != ')')
                            || (x == '{' && i != '}')
                            || (x == '[' && i != ']')
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}
