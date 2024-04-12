#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

fn rand7() -> i32 {
    unimplemented!()
}

impl Solution {
    pub fn resolve() -> i32 {
        (rand7() - 1) * 7 + rand7() - 1
    }

    pub fn rand10() -> i32 {
        let mut pos = Self::resolve();
        while pos >= 40 {
            pos = Self::resolve();
        }

        pos % 10 + 1
    }
}
