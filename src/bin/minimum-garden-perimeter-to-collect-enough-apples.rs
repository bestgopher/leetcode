#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut total = 0;
        for i in 1.. {
            let s = 8 * 3 * (i + 1) * i / 2 - 12 * i;
            total += s;
            if total >= needed_apples {
                return 8 * i;
            }
        }

        unreachable!()
    }
}
