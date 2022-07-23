#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut shift = 0;
        let (mut m, mut n) = (left, right);
        while m < n {
            m >>= 1;
            n >>= 1;
            shift += 1;
        }

        m << shift
    }
}
