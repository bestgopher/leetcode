#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let time = time % ((n - 1) * 2);
        if time < n {
            time + 1
        } else {
            n * 2 - time - 1
        }
    }
}
