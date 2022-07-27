#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::hammingWeight(0b11111111111111111111111111111101),
        31
    );
    assert_eq!(
        Solution::hammingWeight(0b00000000000000000000000000001011),
        3
    );
}

struct Solution;

impl Solution {
    fn hammingWeight(mut num: u32) -> i32 {
        let mut s = 0;

        while num > 0 {
            if num & 1 == 1 {
                s += 1;
            }

            num >>= 1;
        }

        s
    }
}
