#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut right = 0;

        for (i, v) in flips.into_iter().enumerate() {
            right = right.max(v as i32);
            if i as i32 + 1 == right {
                ans += 1;
            }
        }

        ans
    }
}
