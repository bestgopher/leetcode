#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut last = -1;
        for i in s.split(' ').filter_map(|x| x.parse::<i8>().ok()) {
            if i <= last {
                return false;
            }

            last = i;
        }

        true
    }
}
