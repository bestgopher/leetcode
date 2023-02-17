#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    assert_eq!(
        Solution::count_bits(10),
        vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2]
    );
}

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }

        let mut r = vec![0; n as usize + 1];
        let mut start = 0;
        let mut mask = 1;
        for i in 1..=n as usize {
            if i == mask {
                start = 0;
                mask *= 2;
            }

            r[i] = r[start] + 1;
            start += 1;
        }

        r
    }
}
