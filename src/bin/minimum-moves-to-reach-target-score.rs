#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut max_doubles = max_doubles;
        let mut target = target;
        let mut result = 0;
        while target > 1 {
            if target % 2 == 1 {
                target -= 1;
                result += 1;
            } else {
                if max_doubles > 0 {
                    target /= 2;
                    result += 1;
                    max_doubles -= 1;
                } else {
                    result += target - 1;
                    target = 1;
                }
            }
        }

        result
    }
}
